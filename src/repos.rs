//!
//! # Repo handler
//!
//! This module finds all repositories under specified directory

use std::{
  path::{Path, PathBuf},
  process::Command,
};

/// Custom Error if finding repositories fails
#[derive(Debug)]
pub struct PathError {
  error: String,
}

/// Type to handle found repositories
#[derive(Debug)]
pub struct Repos {
  pub repos: Vec<Dir>,
}

/// Type to handle found directories
#[derive(Debug)]
pub struct Dir {
  pub name: String,
  pub path: String,
}

impl Repos {
  /// Function to find all repositories in the specified directory and return the results
  pub fn get_repos(path: String) -> Result<Repos, PathError> {
    let path = Path::new(&path);
    let child_directories = Command::new("ls")
      .current_dir(&path)
      .output()
      .map_err(|e| PathError {
        error: e.to_string(),
      })?;
    let child_directories = String::from_utf8_lossy(&child_directories.stdout);

    let mut status = Vec::new();
    for dir in child_directories.lines() {
      let repo = format!("{}{}/.git", path.display(), dir);
      if PathBuf::from(&repo).is_dir() {
        status.push(Dir {
          name: dir.to_string(),
          path: format!("{}{}", path.display(), dir),
        });
      }
    }
    Ok(Repos { repos: status })
  }
}
