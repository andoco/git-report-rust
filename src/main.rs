mod printer;
mod reporter;
mod scanner;

use clap::Parser;

use reporter::{Git2Reporter, Reporter};
use scanner::{RecursiveScanner, Scanner};
use std::error::Error;

use printer::{Printer, SimplePrinter};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Root path to scan for git repos
    #[clap(short, long, value_parser)]
    path: String,

    /// Folder depth to find git repos at within root path
    #[clap(short, long, value_parser, default_value_t = 0)]
    depth: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let scanner = RecursiveScanner {};
    let repo_paths = scanner.scan(args.path.as_str(), args.depth)?;

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
