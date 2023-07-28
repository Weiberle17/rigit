use std::process::Command;

use crate::repos::{Dir, Repos};

#[derive(Debug)]
pub struct StatusError {
  error: String,
}

#[derive(Debug)]
pub struct Statuses {
  statuses: Vec<Status>,
}
#[derive(Debug)]
pub struct Status {
  pub directory: Dir,
  pub status: Result<String, String>,
}

// TODO: implement proper printing of status
impl Statuses {
  pub fn print(&self) {}
}

pub fn run_status(repos: Repos, verbose: bool) {
  let status = get_status(repos, verbose).unwrap();
  dbg!(status);
}

pub fn get_status(repos: Repos, verbose: bool) -> Result<Statuses, StatusError> {
  let mut statuses: Vec<Status> = Vec::new();
  for dir in repos.repos {
    let status = Command::new("git")
      .arg("status")
      .current_dir(&dir.path)
      .output()
      .map_err(|e| StatusError {
        error: e.to_string(),
      })?;

    let status_output = String::from_utf8_lossy(&status.stdout).to_string();
    let status: Result<String, String>;
    match &status_output {
      x if x.contains("up to date") => {
        if verbose {
          status = Ok(status_output);
        } else {
          status =
            Ok("Local repository is not synchronized with the remote repository.".to_string())
        }
      }
      x if x.contains("modified") => {
        if verbose {
          status = Ok(status_output);
        } else {
          status = Ok("You have uncommited changes in your local repository.".to_string())
        }
      }
      x if x.contains("untracked") => {
        if verbose {
          status = Ok(status_output);
        } else {
          status = Ok("You have untracked files in your repository.".to_string())
        }
      }
      _ => {
        if verbose {
          status = Err(status_output);
        } else {
          status = Err("The repository is clean".to_string());
        }
      }
    }

    statuses.push(Status {
      directory: dir,
      status,
    });
  }
  Ok(Statuses { statuses })
}
