use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use git2::{Branch, BranchType, ErrorClass, ErrorCode, Repository, Status};

#[derive(Debug, Clone)]
pub struct RepoReport {
    pub path: PathBuf,
    pub repo_status: RepoStatus,
    pub branch_status: HashMap<String, BranchStatus>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RepoStatus {
    Clean,
    Dirty,
    NoRepo,
    Error(String),
}

impl std::fmt::Display for RepoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clean => write!(f, "Clean"),
            Self::Dirty => write!(f, "Dirty"),
            Self::NoRepo => write!(f, "None"),
            Self::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BranchStatus {
    Current,
    NoUpstream,
    Ahead,
    Error(String),
}

pub trait Reporter {
    fn report(&self, path: &Path) -> anyhow::Result<RepoReport>;
}

pub struct Git2Reporter {}

impl Git2Reporter {
    pub fn new() -> Git2Reporter {
        Git2Reporter {}
    }
}

impl Reporter for Git2Reporter {
    fn report(&self, path: &Path) -> anyhow::Result<RepoReport> {
        match Repository::open(path) {
            Ok(repo) => {
                let mut status = get_repo_status(&repo);
                let branches = get_branch_statuses(&repo)?;

                if branches.iter().any(|(_, v)| *v != BranchStatus::Current) {
                    status = RepoStatus::Dirty
                }

                Ok(RepoReport {
                    path: path.to_path_buf(),
                    repo_status: status,
                    branch_status: branches,
                })
            }
            Err(error) => match (error.class(), error.code()) {
                (ErrorClass::Repository, ErrorCode::NotFound) => Ok(RepoReport {
                    path: path.to_path_buf(),
                    repo_status: RepoStatus::NoRepo,
                    branch_status: HashMap::new(),
                }),
                _ => Ok(RepoReport {
                    path: path.to_path_buf(),
                    repo_status: RepoStatus::Error(error.to_string()),
                    branch_status: HashMap::new(),
                }),
            },
        }
    }
}

fn get_repo_status(repo: &Repository) -> RepoStatus {
    match repo.statuses(None) {
        Ok(statuses) => {
            let mut report_statuses = statuses
                .iter()
                .map(|s| s.status())
                .map(|s| map_git_status_to_report_status(&s));

            if report_statuses.any(|s| s == RepoStatus::Dirty) {
                return RepoStatus::Dirty;
            }

            return RepoStatus::Clean;
        }
        Err(e) => RepoStatus::Error(e.to_string()),
    }
}

fn get_branch_statuses(repo: &Repository) -> anyhow::Result<HashMap<String, BranchStatus>> {
    match repo.branches(Some(BranchType::Local)) {
        Ok(branches) => {
            let mut branch_changes = HashMap::<String, BranchStatus>::new();

            for b in branches {
                let (b, _) = b?;

                let branch_status = match get_branch_status(repo, &b) {
                    Ok(branch_status) => branch_status,
                    Err(err) => BranchStatus::Error(err.to_string()),
                };

                let branch_name = b
                    .name()
                    .context("could not get branch name")?
                    .ok_or(anyhow!("could not get branch name as utf-8"))?;

                branch_changes.insert(branch_name.to_string(), branch_status);
            }

            Ok(branch_changes)
        }
        Err(e) => Err(anyhow!("failed to get branch statuses: {}", e)),
    }
}

fn get_branch_status(repo: &Repository, b: &Branch) -> anyhow::Result<BranchStatus> {
    if b.upstream().is_err() {
        return Ok(BranchStatus::NoUpstream);
    }

    let local_oid = repo.refname_to_id(b.get().name().context("invalid ref name")?)?;
    let upstream_oid = repo.refname_to_id(
        b.upstream()?
            .get()
            .name()
            .ok_or(anyhow!("invalid ref name"))?,
    )?;

    let ahead_behind = repo.graph_ahead_behind(local_oid, upstream_oid);

    match ahead_behind {
        Ok((ahead, _)) => {
            if ahead > 0 {
                Ok(BranchStatus::Ahead)
            } else {
                Ok(BranchStatus::Current)
            }
        }
        Err(e) => Ok(BranchStatus::Error(e.to_string())),
    }
}

fn map_git_status_to_report_status(status: &Status) -> RepoStatus {
    let changed_mask = Status::INDEX_NEW
        | Status::INDEX_MODIFIED
        | Status::INDEX_DELETED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE
        | Status::WT_NEW
        | Status::WT_MODIFIED
        | Status::WT_DELETED
        | Status::WT_RENAMED
        | Status::WT_TYPECHANGE;

    let has_changes = *status & changed_mask == *status;

    if has_changes {
        return RepoStatus::Dirty;
    }

    return RepoStatus::Clean;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_git_status_to_report_status() {
        let expected_changed_statuses = [
            Status::INDEX_NEW,
            Status::INDEX_MODIFIED,
            Status::INDEX_DELETED,
            Status::INDEX_RENAMED,
            Status::INDEX_TYPECHANGE,
            Status::WT_NEW,
            Status::WT_MODIFIED,
            Status::WT_DELETED,
            Status::WT_RENAMED,
            Status::WT_TYPECHANGE,
        ];

        for status in expected_changed_statuses.iter() {
            assert_eq!(RepoStatus::Dirty, map_git_status_to_report_status(status))
        }
    }
}
