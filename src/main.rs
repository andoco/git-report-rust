use std::env;

use git2::{Repository, Status};

fn main() {
    let args: Vec<String> = env::args().collect();
    let repo_path = &args[1];

    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let ok_statuses = [Status::CURRENT, Status::IGNORED];

    if let Ok(statuses) = repo.statuses(None) {
        for entry in statuses.iter() {
            let status = entry.status();
            if !ok_statuses.contains(&status) {
                if let Some(path) = entry.path() {
                    println!("{} = {:?}", path, status);
                }
            }
        }
    } else {
        println!("Failed to get statuses from repo");
    };
}
