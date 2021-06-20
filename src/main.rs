use std::env;

use git2::{Repository, Status};
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let root_path = &args[1];

    for entry in fs::read_dir(root_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let path = path.to_str().ok_or("Not a valid path")?;

            let _ = Repository::open(path).and_then(is_dirty).map_or_else(
                |err| println!("{} ERROR {}", path, err),
                |dirty| print_report(path, dirty),
            );
        }
    }

    Ok(())
}

fn is_dirty(repo: Repository) -> Result<bool, git2::Error> {
    let ok_statuses = [Status::CURRENT, Status::IGNORED];
    let statuses = repo.statuses(None)?;

    let result = statuses
        .iter()
        .any(|entry| !ok_statuses.contains(&entry.status()));

    Ok(result)
}

fn print_report(path: &str, dirty: bool) {
    if dirty {
        println!("{} is DIRTY", path);
    } else {
        println!("{} is CLEAN", path);
    }
}
