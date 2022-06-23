mod cli;
mod reporter;
mod visitor;

use cli::get_args;
use reporter::{Git2Reporter, Reporter};

use std::{env::current_dir, error::Error, path::Path};
use visitor::{SimpleWalker, State, Walker};

use crate::visitor::Status;

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    let path = &args.path.unwrap_or(current_dir().unwrap());
    let reporter = Git2Reporter {};
    let walker = SimpleWalker::new();

    let mut visitor = |path: &Path, state: &State| {
        let report = reporter.report(path).unwrap();
        for (i, (name, status)) in report.branch_status.iter().enumerate() {
            if i < report.branch_status.len() - 1 {
                state.extend(Status::Open).print();
            } else {
                state.extend(Status::Terminal).print();
            }
            println!("{} - {:?}", name, status);
        }
    };

    walker.walk(path, args.depth, State::new(), &mut visitor);

    Ok(())
}
