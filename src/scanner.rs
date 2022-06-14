use std::{
    error::Error,
    fmt,
    fs::read_dir,
    path::{Path, PathBuf},
    vec,
};

#[derive(Debug)]
pub struct ScannerError {
    pub cause: Box<dyn Error>,
}

impl Error for ScannerError {}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error scanning for folders: {}", self.cause)
    }
}

impl From<std::io::Error> for ScannerError {
    fn from(error: std::io::Error) -> Self {
        ScannerError {
            cause: Box::new(error),
        }
    }
}

pub trait Scanner {
    fn scan(&self, root: &Path, depth: u8) -> Result<Vec<PathBuf>, ScannerError>;
}

pub struct RecursiveScanner;

impl Scanner for RecursiveScanner {
    fn scan(&self, root: &Path, depth: u8) -> Result<Vec<PathBuf>, ScannerError> {
        let mut repos: Vec<PathBuf> = vec![];
        let dir_entries = read_dir(root)?;

        for entry in dir_entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if depth == 0 {
                        repos.push(path);
                    } else {
                        let mut child_repos = self.scan(&path, depth - 1)?;
                        repos.append(&mut child_repos);
                    }
                }
            }
        }

        return Ok(repos);
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, File};

    use super::*;

    use git2::Repository;

    #[test]
    fn test_scan_when_empty_detects_no_repos() {
        let temp_root = tempfile::tempdir().unwrap();
        let root = temp_root.path();

        let scanner = RecursiveScanner {};

        let result = scanner.scan(&root, 0).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_scan_when_one_repo_detects_one_repo() {
        let temp_root = tempfile::tempdir().unwrap();
        let root = temp_root.path();

        Repository::init(&root.join("repo1")).unwrap();

        let scanner = RecursiveScanner {};

        let result = scanner.scan(&root, 0).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], root.join("repo1"));
    }

    #[test]
    fn test_scan_when_two_repos_detects_two_repos() {
        let temp_root = tempfile::tempdir().unwrap();
        let root = temp_root.path();

        Repository::init(&root.join("repo1")).unwrap();
        Repository::init(&root.join("repo2")).unwrap();

        let scanner = RecursiveScanner {};

        let result = scanner.scan(&root, 0).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], root.join("repo1"));
        assert_eq!(result[1], root.join("repo2"));
    }

    #[test]
    fn test_scan_when_repos_and_non_repos_detects_both() {
        let temp_root = tempfile::tempdir().unwrap();
        let root = temp_root.path();

        create_dir(root.join("non-repo")).unwrap();
        File::create(root.join("file1.txt")).unwrap();
        Repository::init(&root.join("repo1")).unwrap();
        Repository::init(&root.join("repo2")).unwrap();

        let scanner = RecursiveScanner {};

        let result = scanner.scan(&root, 0).unwrap();

        assert_eq!(result.len(), 3);
        assert!(result.contains(&root.join("non-repo")));
        assert!(result.contains(&root.join("repo1")));
        assert!(result.contains(&root.join("repo2")));
    }

    #[test]
    fn test_scan_when_nested_repos_detects_all_repos() {
        let temp_root = tempfile::tempdir().unwrap();
        let root = temp_root.path();

        create_dir(root.join("dir1")).unwrap();
        Repository::init(&root.join("dir1").join("repo1")).unwrap();

        create_dir(root.join("dir2")).unwrap();
        Repository::init(&root.join("dir2").join("repo2")).unwrap();

        let scanner = RecursiveScanner {};

        let result = scanner.scan(&root, 1).unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.contains(&root.join("dir1").join("repo1")));
        assert!(result.contains(&root.join("dir2").join("repo2")));
    }
}
