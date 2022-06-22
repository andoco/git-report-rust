use std::{io::Write, path::Path};

use colored::Colorize;

use crate::reporter::{RepoReport, RepoStatus};

pub trait Printer {
    fn print_report(self, path: &Path, report: RepoReport, buf: impl Write);
}

pub struct SimplePrinter;

impl Printer for SimplePrinter {
    fn print_report(self, path: &Path, report: RepoReport, mut buf: impl Write) {
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
                buf.write_fmt(format_args!(
                    "{} {} [{}]",
                    repo_status,
                    path.display(),
                    branch_statuses.join(", ")
                ))
                .unwrap();
            }
            _ => {
                buf.write_fmt(format_args!("{} {}", repo_status, path.display()))
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use colored::Colorize;

    use crate::reporter::BranchStatus;

    use super::*;

    fn setup() -> (SimplePrinter, Vec<u8>) {
        (SimplePrinter, Vec::new())
    }

    fn assert_output(buf: &Vec<u8>, expected: String) {
        let output = std::str::from_utf8(buf.as_slice()).unwrap().to_string();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_print_report_when_clean() {
        let (printer, mut buf) = setup();

        let report = RepoReport {
            repo_status: RepoStatus::Clean,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        printer.print_report(PathBuf::from("./repos/repo").as_path(), report, &mut buf);

        assert_output(
            &buf,
            format!("{} ./repos/repo [master:Current]", "Clean".green()),
        );
    }

    #[test]
    fn test_print_report_when_dirty() {
        let (printer, mut buf) = setup();

        let report = RepoReport {
            repo_status: RepoStatus::Dirty,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        printer.print_report(PathBuf::from("./repos/repo").as_path(), report, &mut buf);

        assert_output(
            &buf,
            format!("{} ./repos/repo [master:Current]", "Dirty".red()),
        );
    }

    #[test]
    fn test_print_report_when_no_repo() {
        let (printer, mut buf) = setup();

        let report = RepoReport {
            repo_status: RepoStatus::NoRepo,
            branch_status: HashMap::new(),
        };

        printer.print_report(PathBuf::from("./repos/repo").as_path(), report, &mut buf);

        assert_output(&buf, format!("{} ./repos/repo", "None".yellow()));
    }

    #[test]
    fn test_print_report_when_unpushed_branch() {
        let (printer, mut buf) = setup();

        let report = RepoReport {
            repo_status: RepoStatus::Dirty,
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Ahead)]),
        };

        printer.print_report(PathBuf::from("./repos/repo").as_path(), report, &mut buf);

        assert_output(
            &buf,
            format!("{} ./repos/repo [master:Ahead]", "Dirty".red()),
        );
    }

    #[test]
    fn test_print_report_when_error() {
        let (printer, mut buf) = setup();

        let report = RepoReport {
            repo_status: RepoStatus::Error("Some error".to_string()),
            branch_status: HashMap::from([("master".to_string(), BranchStatus::Current)]),
        };

        printer.print_report(PathBuf::from("./repos/repo").as_path(), report, &mut buf);

        assert_output(&buf, format!("{} ./repos/repo", "Some error".red()));
    }

    #[test]
    fn test_print_report_when_untracked_branch() {
        let (printer, mut buf) = setup();

        let report = RepoReport {
            repo_status: RepoStatus::Dirty,
            branch_status: HashMap::from([("feature-1".to_string(), BranchStatus::NoUpstream)]),
        };

        printer.print_report(PathBuf::from("./repos/repo").as_path(), report, &mut buf);

        assert_output(
            &buf,
            format!("{} ./repos/repo [feature-1:NoUpstream]", "Dirty".red()),
        );
    }
}
