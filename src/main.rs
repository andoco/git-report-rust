mod cli;
mod printer;
mod reporter;
mod scanner;

use cli::get_args;
use reporter::{Git2Reporter, Reporter};
use scanner::{RecursiveScanner, Scanner};
use std::error::Error;

use printer::{Printer, SimplePrinter};

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();

    let scanner = RecursiveScanner {};
    let repo_paths = scanner.scan(&args.path.unwrap(), args.depth)?;

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
