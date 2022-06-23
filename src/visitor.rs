use std::{fs::read_dir, path::Path};

pub trait Walker {
    fn walk(&self, root: &Path, depth: u8, state: State, visitor: &mut dyn FnMut(&Path, &State));
}

pub struct SimpleWalker;

impl SimpleWalker {
    pub fn new() -> SimpleWalker {
        SimpleWalker {}
    }
}

impl Walker for SimpleWalker {
    fn walk(&self, root: &Path, depth: u8, state: State, visitor: &mut dyn FnMut(&Path, &State)) {
        state.print();
        println!("{}", root.file_name().unwrap().to_str().unwrap());

        if depth == 0 {
            visitor(&root, &state);
            return;
        }

        let dir_entries: Vec<_> = read_dir(root).unwrap().collect();

        for (i, entry) in dir_entries.iter().enumerate() {
            if let Ok(entry) = entry {
                let path = entry.path();

                let new_state = match i {
                    i if i == dir_entries.len() - 1 => state.extend(Status::Terminal),
                    _ => state.extend(Status::Open),
                };
                self.walk(&path, depth - 1, new_state, visitor);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Status {
    Open,
    Continue,
    Terminal,
    Empty,
}

pub struct State {
    status: Vec<Status>,
}

impl State {
    pub fn new() -> State {
        State { status: Vec::new() }
    }

    pub fn extend(&self, status: Status) -> State {
        let mut new_status: Vec<Status> = self
            .status
            .iter()
            .map(|status| match status {
                Status::Open => Status::Continue,
                Status::Continue => Status::Continue,
                Status::Terminal => Status::Empty,
                Status::Empty => Status::Empty,
            })
            .collect();

        new_status.push(status);

        State { status: new_status }
    }

    pub fn print(&self) {
        self.status.iter().for_each(|status| {
            let s = match *status {
                Status::Open => "├──",
                Status::Continue => "│  ",
                Status::Terminal => "└──",
                Status::Empty => "   ",
            };
            print!("{}", s);
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::create_dir_all, path::PathBuf};

    use super::*;

    #[test]
    fn test_walk_visits_paths_at_depth() {
        let temp_root = tempfile::tempdir().unwrap();
        let root = temp_root.path();

        let mut visited: Vec<PathBuf> = Vec::new();

        let repos = root.join("repos");
        let github = repos.join("github.com");
        let me = github.join("me");
        let repo1 = me.join("repo1");

        let bitbucket = repos.join("bitbucket.com");
        let me_2 = bitbucket.join("me");
        let repo1_2 = me_2.join("repo1");

        create_dir_all(&repo1).unwrap();
        create_dir_all(&repo1_2).unwrap();

        let walker = SimpleWalker::new();
        let state = State::new();

        let mut visitor = |path: &Path, _state: &State| {
            visited.push(path.to_path_buf());
        };

        walker.walk(&root, 4, state, &mut visitor);

        assert_eq!(visited, vec![repo1, repo1_2]);
    }
}
