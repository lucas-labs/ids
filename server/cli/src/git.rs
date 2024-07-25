//! ### Git
//!
//! The `git` module is responsible for interacting with the git repository, and providing the
//! necessary information to the server.
//!
//! It's just simple wrapper around some git cli commands.

use {
    eyre::Result,
    lool::{cli::stylize::Stylize, fail, s},
    std::{
        fmt::Display,
        path::{Path, PathBuf},
    },
};

pub enum GitStatus {
    Untracked,
    Modified,
    Deleted,
    Added,
    Other,
}

impl Display for GitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match &self {
            GitStatus::Untracked => s!("new"),
            GitStatus::Modified => s!("mod"),
            GitStatus::Deleted => s!("del"),
            GitStatus::Added => s!("add"),
            GitStatus::Other => s!("???"),
        };
        write!(f, "{}", status)
    }
}

/// Stores the status of a file in the repository as returned by `git status --porcelain`
pub struct GitFileStatus {
    pub status: GitStatus,
    pub path: PathBuf,
    pub link: PathBuf,
}

pub struct OrganizedStatus {
    pub dir: String,
    pub files: Vec<GitFileStatus>,
}

impl Display for OrganizedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{}\n{}", self.dir, "-".repeat(self.dir.len())).magenta());
        for file in &self.files {
            let stat = match file.status {
                GitStatus::Untracked => file.status.to_string().green(),
                GitStatus::Modified => file.status.to_string().yellow(),
                GitStatus::Deleted => file.status.to_string().red(),
                GitStatus::Added => file.status.to_string().blue(),
                GitStatus::Other => file.status.to_string(),
            };

            result.push_str(&format!("\n  {} {}", stat, file.path.display()));
        }
        write!(f, "{}", result)
    }
}

/// Check if the given path is a git repository
pub fn get_repo_top_level(path: &PathBuf) -> Result<PathBuf> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .current_dir(path)
        .output()
        .expect("Failed to check if path is a git repository");

    if output.status.success() {
        let output = String::from_utf8(output.stdout).expect("Failed to parse git output");
        Ok(PathBuf::from(output.trim()))
    } else {
        fail!("{} is not in a git repo", path.display())
    }
}

/// Get the current branch of the repository
///
/// **Arguments**
///
/// * `repoPath` - The path to the repository
/// * `ext_filter` - An optional filter for file extensions
pub fn status(repo_path: &Path, ext_filter: Option<Vec<&str>>) -> Vec<OrganizedStatus> {
    let output = std::process::Command::new("git")
        .arg("status")
        .arg("-uall")
        .arg("--porcelain")
        .current_dir(repo_path)
        .output()
        .expect("Failed to get git status");

    let output = String::from_utf8(output.stdout).expect("Failed to parse git status output");
    let mut files = Vec::new();

    // parse the output of git status line by line
    for line in output.lines() {
        let mut parts = line.split_whitespace();
        let status = parts.next().expect("Failed to parse git status");
        let path = parts.next().expect("Failed to parse git status");

        if let Some(exts) = &ext_filter {
            if !exts.iter().any(|ext| path.ends_with(ext)) {
                continue;
            }
        }

        let status = if status.starts_with('M') || status.ends_with('M') {
            GitStatus::Modified
        } else if status.starts_with('D') || status.ends_with('D') {
            GitStatus::Deleted
        } else if status.starts_with("??") {
            GitStatus::Untracked
        } else if status == "A" {
            GitStatus::Added
        } else {
            GitStatus::Other
        };

        let link = PathBuf::from(path);
        let path = repo_path.join(path.trim_matches('"'));
        let path = std::path::absolute(path).expect("Failed to get absolute path");

        files.push(GitFileStatus { status, path, link });
    }

    organize_status(files)
}

/// receives a list of files and groups them them by their immediate parent directory
fn organize_status(diff: Vec<GitFileStatus>) -> Vec<OrganizedStatus> {
    let mut organized: Vec<OrganizedStatus> = Vec::new();

    for file in diff {
        let parent = file.path.parent().expect("Failed to get parent directory");
        let parent = parent.file_name().expect("Failed to get parent directory name");
        let parent = parent.to_str().expect("Failed to convert parent directory to string");

        if let Some(dir) = organized.iter_mut().find(|dir| dir.dir == parent) {
            dir.files.push(file);
        } else {
            organized.push(OrganizedStatus {
                dir: parent.to_string(),
                files: vec![file],
            });
        }
    }

    organized
}
