use std::{fs::read_dir, io::Write, path::Path};

use anyhow::anyhow;
use colored::{ColoredString, Colorize};

use crate::{
    print_stack::{Node, PrintStack},
    repo::{RepoReport, RepoStatus, Reporter},
};

pub trait Walker {
    fn report(&self, root: &Path, depth: u8, out: &mut dyn Write) -> anyhow::Result<()>;
}

pub struct SimpleWalker<'a> {
    reporter: &'a dyn Reporter,
}

impl<'a> SimpleWalker<'a> {
    pub fn new(reporter: &'a dyn Reporter) -> SimpleWalker<'a> {
        SimpleWalker { reporter }
    }

    fn visit(&self, path: &Path, stack: &mut PrintStack) -> anyhow::Result<()> {
        let report = self.reporter.report(path)?;

        for (i, (name, status)) in report.branch_status.iter().enumerate() {
            let txt = format!("{} - {:?}", name, status);
            if i < report.branch_status.len() - 1 {
                stack.extend(Node::Open(txt)).print()?;
            } else {
                stack.extend(Node::Terminal(txt)).print()?;
            }
        }

        Ok(())
    }

    fn walk(&self, root: &Path, depth: u8, stack: &mut PrintStack) -> anyhow::Result<()> {
        stack.print()?;

        if depth == 0 {
            self.visit(&root, stack)?;
            return Ok(());
        }

        let dir_entries: Vec<_> = read_dir(root)?.collect();

        for (i, entry) in dir_entries.iter().enumerate() {
            if let Ok(entry) = entry {
                let path = entry.path();
                let report = self.reporter.report(&path).unwrap();
                let name = get_name(&report)?;

                let mut new_stack = match i {
                    i if i == dir_entries.len() - 1 => stack.extend(Node::Terminal(name)),
                    _ => stack.extend(Node::Open(name)),
                };
                self.walk(&path, depth - 1, &mut new_stack)?;
            }
        }

        Ok(())
    }
}

impl Walker for SimpleWalker<'_> {
    fn report(&self, root: &Path, depth: u8, out: &mut dyn Write) -> anyhow::Result<()> {
        write!(
            out,
            "{}",
            root.to_str()
                .ok_or(anyhow!("could not get root as string"))?
        )?;
        self.walk(root, depth, &mut PrintStack::new(out))?;
        Ok(())
    }
}

fn get_name(report: &RepoReport) -> anyhow::Result<String> {
    let name = report
        .path
        .file_name()
        .ok_or(anyhow!("cannot get file name"))?
        .to_str()
        .ok_or(anyhow!("cannot get file name as a string"))?;

    let colored_name = match &report.repo_status {
        RepoStatus::Clean => name.green(),
        RepoStatus::Dirty => name.red(),
        RepoStatus::NoRepo => ColoredString::from(name),
        RepoStatus::Error(err) => format!("{} (ERR: {})", name, err).red(),
    }
    .to_string();

    Ok(colored_name)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::create_dir_all, path::PathBuf, str::from_utf8};

    use crate::repo::{BranchStatus, Git2Reporter};

    use super::*;

    #[test]
    fn test_get_name_when_clean_repo_is_green() {
        let report = RepoReport {
            path: PathBuf::from("./repos/repo1"),
            repo_status: RepoStatus::Clean,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let name = get_name(&report).unwrap();

        assert_eq!(name, "repo1".green().to_string());
    }

    #[test]
    fn test_get_name_when_dirty_repo_is_red() {
        let report = RepoReport {
            path: PathBuf::from("./repos/repo1"),
            repo_status: RepoStatus::Dirty,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let name = get_name(&report).unwrap();

        assert_eq!(name, "repo1".red().to_string());
    }

    #[test]
    fn test_get_name_when_no_repo_is_normal_color() {
        let report = RepoReport {
            path: PathBuf::from("./repos/repo1"),
            repo_status: RepoStatus::NoRepo,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let name = get_name(&report).unwrap();

        assert_eq!(name, "repo1".to_string());
    }

    #[test]
    fn test_get_name_when_error_is_red() {
        let report = RepoReport {
            path: PathBuf::from("./repos/repo1"),
            repo_status: RepoStatus::Error("Some error".to_string()),
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let name = get_name(&report).unwrap();

        assert_eq!(name, "repo1 (ERR: Some error)".red().to_string());
    }

    #[test]
    fn test_walk_visits_paths_at_depth() {
        let temp_root = tempfile::tempdir().unwrap();
        let root = temp_root.path();

        let repos = root.join("repos");
        let github = repos.join("github.com");
        let me = github.join("me");
        let repo1 = me.join("repo1");

        let bitbucket = repos.join("bitbucket.com");
        let me_2 = bitbucket.join("me");
        let repo1_2 = me_2.join("repo1");

        create_dir_all(&repo1).unwrap();
        create_dir_all(&repo1_2).unwrap();

        let reporter = Git2Reporter::new();

        let walker = SimpleWalker::new(&reporter);

        let mut out = Vec::<u8>::new();

        walker.report(&root, 4, &mut out).unwrap();

        let printed = from_utf8(&out).unwrap();

        assert_eq!(printed, format!("{}\n└── repos\n    ├── github.com\n    │   └── me\n    │       └── repo1\n    └── bitbucket.com\n        └── me\n            └── repo1\n", root.to_str().unwrap()))
    }
}
