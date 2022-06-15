mod printer;
mod reporter;
mod scanner;

use clap::{arg, command, value_parser};

use reporter::{Git2Reporter, Reporter};
use scanner::{RecursiveScanner, Scanner};
use std::{error::Error, path::PathBuf};

use printer::{Printer, SimplePrinter};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .arg(
            arg!([NAME] "Root path to scan for git repos")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-d --depth <DEPTH> "Folder depth to find git repos at within root path")
                .value_parser(value_parser!(u8))
                .required(false)
                .default_value("0"),
        )
        .get_matches();

    let root_path = matches.get_one::<PathBuf>("NAME").unwrap();
    let depth = matches.get_one::<u8>("depth").unwrap();

    let scanner = RecursiveScanner {};
    let repo_paths = scanner.scan(root_path, *depth)?;

    let reporter = Git2Reporter {};

    for path in repo_paths {
        let report = reporter.report(path.as_path());

        match report {
            Ok(report) => {
                let printer = SimplePrinter;
                println!("{}", printer.print_report(&path, report));
            }
            Err(err) => println!("{:?} ERROR: {}", path, err),
        }
    }

    Ok(())
}
