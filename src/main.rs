mod cli;
mod printer;
mod reporter;
mod scanner;

use cli::get_args;
use reporter::{Git2Reporter, Reporter};
use scanner::{RecursiveScanner, Scanner};
use std::{env::current_dir, error::Error};

use printer::{Printer, SimplePrinter};

use crate::reporter::RepoReport;

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();

    let scanner = RecursiveScanner {};
    let repo_paths = scanner.scan(&args.path.unwrap_or(current_dir().unwrap()), args.depth)?;

    let reporter = Git2Reporter {};
    let reports: Vec<RepoReport> = repo_paths
        .iter()
        .map(|path| reporter.report(path).unwrap())
        .collect();

    let printer = SimplePrinter;
    let mut buf: Vec<u8> = Vec::new();
    printer.print_report(reports, &mut buf);

    let output = std::str::from_utf8(buf.as_slice()).unwrap().to_string();
    println!("{}", output);

    Ok(())
}
