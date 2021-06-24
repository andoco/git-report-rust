use std::env;

use ansi_term::Colour;
use git2::{ErrorCode, Repository, Status};
use std::{error::Error, fs};

#[derive(Debug, PartialEq)]
enum RepoStatus {
    Clean,
    Changed,
    NotRepo,
    Error(String),
}

impl std::fmt::Display for RepoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clean => write!(f, "Clean"),
            Self::Changed => write!(f, "Changed"),
            Self::NotRepo => write!(f, "Not a repo"),
            Self::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let root_path = &args[1];

    let mut statuses: Vec<(String, RepoStatus)> = vec![];

    for entry in fs::read_dir(root_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let path = path.to_str().ok_or("Not a valid path")?;

            let status = Repository::open(path)
                .map_or_else(map_git_error_to_repo_status, get_repo_report_status);

            statuses.push((path.to_string(), status));
        }
    }

    let max_length = statuses.iter().map(|(p, _)| p.len()).max().unwrap();

    statuses
        .iter()
        .for_each(|(p, s)| print_status(p, s, max_length));

    Ok(())
}

fn print_status(path: &String, status: &RepoStatus, column_size: usize) {
    let to_print = match status {
        RepoStatus::Clean => Colour::Green.paint("Clean"),
        RepoStatus::Changed => Colour::Purple.paint("Changed"),
        RepoStatus::NotRepo => Colour::Yellow.paint("Not a repo"),
        RepoStatus::Error(e) => Colour::Red.paint(format!("Error {}", e)),
    };
    println!(
        "{}{: <width$}{}",
        path,
        " ",
        to_print,
        width = column_size - path.len() + 1,
    );
}

fn get_repo_report_status(repo: Repository) -> RepoStatus {
    match repo.statuses(None) {
        Ok(statuses) => {
            let mut report_statuses = statuses
                .iter()
                .map(|s| s.status())
                .map(|s| map_git_status_to_report_status(&s));

            if report_statuses.any(|s| s == RepoStatus::Changed) {
                return RepoStatus::Changed;
            }

            return RepoStatus::Clean;
        }
        Err(e) => RepoStatus::Error(e.to_string()),
    }
}

fn map_git_error_to_repo_status(err: git2::Error) -> RepoStatus {
    if err.code() == ErrorCode::NotFound {
        return RepoStatus::NotRepo;
    }
    RepoStatus::Error(err.to_string())
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
        return RepoStatus::Changed;
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
            assert_eq!(RepoStatus::Changed, map_git_status_to_report_status(status))
        }
    }
}
