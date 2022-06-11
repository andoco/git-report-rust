use std::fmt::Debug;

use colored::Colorize;

use crate::{RepoReport, RepoStatus};

pub trait Printer {
    fn print_report(self, path: &str, report: RepoReport) -> String;
}

pub struct SimplePrinter;

impl Printer for SimplePrinter {
    fn print_report(self, path: &str, report: RepoReport) -> String {
        let repo_status = match &report.repo_status {
            RepoStatus::Clean => RepoStatus::Clean.to_string().green(),
            RepoStatus::Dirty => RepoStatus::Dirty.to_string().red(),
            RepoStatus::NoRepo => RepoStatus::NoRepo.to_string().yellow(),
            RepoStatus::Error(s) => s.red(),
        }
        .to_string();

        let branch_statuses: Vec<String> = report
            .branch_status
            .iter()
            .map(|(k, v)| match v {
                x => format!("{}:{:?}", k, x),
            })
            .collect();

        match &report.repo_status {
            RepoStatus::Clean | RepoStatus::Dirty => {
                format!("{} {} [{}]", repo_status, path, branch_statuses.join(", "))
            }
            _ => format!("{} {}", repo_status, path),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use colored::Colorize;

    use crate::{BranchStatus, RepoStatus};

    use super::*;

    #[test]
    fn test_print_report_when_clean() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Clean,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let result = printer.print_report("./repos/repo", report);

        assert_eq!(
            result,
            format!("{} ./repos/repo [master:Current]", "Clean".green())
        );
    }

    #[test]
    fn test_print_report_when_dirty() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Dirty,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let result = printer.print_report("./repos/repo", report);

        assert_eq!(
            result,
            format!("{} ./repos/repo [master:Current]", "Dirty".red())
        );
    }

    #[test]
    fn test_print_report_when_no_repo() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::NoRepo,
            branch_status: HashMap::new(),
        };

        let result = printer.print_report("./repos/repo", report);

        assert_eq!(result, format!("{} ./repos/repo", "Not a repo".yellow()));
    }

    #[test]
    fn test_print_report_when_unpushed_branch() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Dirty,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Ahead)]),
        };

        let result = printer.print_report("./repos/repo", report);

        assert_eq!(
            result,
            format!("{} ./repos/repo [master:Ahead]", "Dirty".red())
        );
    }

    #[test]
    fn test_print_report_when_error() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Error("Some error".to_string()),
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let result = printer.print_report("./repos/repo", report);

        assert_eq!(result, format!("{} ./repos/repo", "Some error".red()));
    }

    #[test]
    fn test_print_report_when_untracked_branch() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Dirty,
            branch_status: HashMap::from([("feature-1".to_string(), BranchStatus::NoUpstream)]),
        };

        let result = printer.print_report("./repos/repo", report);

        assert_eq!(
            result,
            format!("{} ./repos/repo [feature-1:NoUpstream]", "Dirty".red())
        );
    }
}
