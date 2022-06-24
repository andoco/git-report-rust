use std::{fs::read_dir, path::Path};

use crate::print_stack::{Node, PrintStack};

pub trait Walker {
    fn walk(
        &self,
        root: &Path,
        depth: u8,
        stack: PrintStack,
        visitor: &mut dyn FnMut(&Path, &PrintStack),
    );
}

pub struct SimpleWalker;

impl SimpleWalker {
    pub fn new() -> SimpleWalker {
        SimpleWalker {}
    }
}

impl Walker for SimpleWalker {
    fn walk(
        &self,
        root: &Path,
        depth: u8,
        stack: PrintStack,
        visitor: &mut dyn FnMut(&Path, &PrintStack),
    ) {
        stack.print(std::io::stdout());

        if depth == 0 {
            visitor(&root, &stack);
            return;
        }

        let dir_entries: Vec<_> = read_dir(root).unwrap().collect();

        for (i, entry) in dir_entries.iter().enumerate() {
            if let Ok(entry) = entry {
                let path = entry.path();
                let name = format!("{}", path.file_name().unwrap().to_str().unwrap());

                let new_stack = match i {
                    i if i == dir_entries.len() - 1 => stack.extend(Node::Terminal(name)),
                    _ => stack.extend(Node::Open(name)),
                };
                self.walk(&path, depth - 1, new_stack, visitor);
            }
        }
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
        let stack = PrintStack::new();

        let mut visitor = |path: &Path, _stack: &PrintStack| {
            visited.push(path.to_path_buf());
        };

        walker.walk(&root, 4, stack, &mut visitor);

        assert_eq!(visited, vec![repo1, repo1_2]);
    }
}
