mod cli;
mod printer;
mod reporter;
mod scanner;

use cli::get_args;
use reporter::{Git2Reporter, Reporter};
use scanner::{RecursiveScanner, Scanner};
use std::{env::current_dir, error::Error};

use printer::{Printer, SimplePrinter};

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();

    let scanner = RecursiveScanner {};
    let repo_paths = scanner.scan(&args.path.unwrap_or(current_dir().unwrap()), args.depth)?;

    let reporter = Git2Reporter {};

    for path in repo_paths {
        let report = reporter.report(path.as_path());

        let mut buf: Vec<u8> = Vec::new();

        match report {
            Ok(report) => {
                let printer = SimplePrinter;
                printer.print_report(&path, report, &mut buf);
            }
            Err(err) => println!("{:?} ERROR: {}", path, err),
        }

        let output = std::str::from_utf8(buf.as_slice()).unwrap().to_string();

        println!("{}", output);
    }

    Ok(())
}
