use std::path::PathBuf;

use clap::{arg, command, value_parser};

pub struct Args {
    pub path: Option<PathBuf>,
    pub depth: u8,
}

pub fn get_args() -> Args {
    let matches = command!()
        .arg(
            arg!([NAME] "Root path to scan for git repos")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-d --depth <DEPTH> "Folder depth to find git repos at within root path")
                .value_parser(value_parser!(u8))
                .required(false)
                .default_value("0"),
        )
        .get_matches();

    Args {
        path: matches.get_one::<PathBuf>("NAME").cloned(),
        depth: matches.get_one::<u8>("depth").unwrap().to_owned(),
    }
}
