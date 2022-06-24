mod cli;
mod print_stack;
mod reporter;
mod visitor;

use cli::get_args;
use print_stack::PrintStack;
use reporter::{Git2Reporter, Reporter};

use std::{env::current_dir, error::Error, path::Path};
use visitor::{SimpleWalker, Walker};

use crate::print_stack::Node;

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    let path = &args.path.unwrap_or(current_dir().unwrap());
    let reporter = Git2Reporter {};
    let walker = SimpleWalker::new();

    let mut visitor = |path: &Path, stack: &PrintStack| {
        let report = reporter.report(path).unwrap();
        for (i, (name, status)) in report.branch_status.iter().enumerate() {
            let txt = format!("{} - {:?}", name, status);
            if i < report.branch_status.len() - 1 {
                stack.extend(Node::Open(txt)).print(std::io::stdout());
            } else {
                stack.extend(Node::Terminal(txt)).print(std::io::stdout());
            }
            println!();
        }
    };

    walker.walk(path, args.depth, PrintStack::new(), &mut visitor);

    Ok(())
}
