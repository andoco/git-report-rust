mod cli;
mod print_stack;
mod repo;
mod walker;

use cli::get_args;
use repo::Git2Reporter;

use std::env::current_dir;
use walker::{SimpleWalker, Walker};

fn main() -> anyhow::Result<()> {
    let args = get_args();
    let path = &args.path.unwrap_or(current_dir()?);
    let reporter = Git2Reporter::new();
    let mut out = std::io::stdout();
    let walker = SimpleWalker::new(&reporter);

    walker.report(path, args.depth, &mut out)?;

    Ok(())
}
