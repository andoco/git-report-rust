use crate::{RepoReport, RepoStatus};

pub trait Printer {
    fn print_report(self, report: RepoReport) -> String;
}

pub struct SimplePrinter;

impl Printer for SimplePrinter {
    fn print_report(self, report: RepoReport) -> String {
        let repo_status = match report.repo_status {
            RepoStatus::Error(s) => s,
            status => format!("{:?}", status),
        };

        let branch_statuses: Vec<String> = report
            .branch_status
            .iter()
            .map(|(k, v)| match v {
                x => format!("{}:{:?}", k, x),
            })
            .collect();

        format!("{} | {}", repo_status, branch_statuses.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{BranchStatus, RepoStatus};

    use super::*;

    #[test]
    fn test_print_report_when_clean() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Clean,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let result = printer.print_report(report);

        assert_eq!(result, "Clean | master:Current");
    }

    #[test]
    fn test_print_report_when_changed() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Changed,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let result = printer.print_report(report);

        assert_eq!(result, "Changed | master:Current");
    }

    #[test]
    fn test_print_report_when_unpushed() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Unpushed,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let result = printer.print_report(report);

        assert_eq!(result, "Unpushed | master:Current");
    }

    #[test]
    fn test_print_report_when_not_repo() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::NotRepo,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let result = printer.print_report(report);

        assert_eq!(result, "NotRepo | master:Current");
    }

    #[test]
    fn test_print_report_when_error() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Error("Some error".to_string()),
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        let result = printer.print_report(report);

        assert_eq!(result, "Some error | master:Current");
    }

    #[test]
    fn test_print_report_when_untracked_branch() {
        let printer = SimplePrinter;

        let report = RepoReport {
            repo_status: RepoStatus::Clean,
            branch_status: HashMap::from([("feature-1".to_string(), BranchStatus::NoUpstream)]),
        };

        let result = printer.print_report(report);

        assert_eq!(result, "Clean | feature-1:NoUpstream");
    }
}
