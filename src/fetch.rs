use std::process::Command;

use crate::repos::{Dir, Repos};

#[derive(Debug)]
pub struct FetchError {
  error: String,
}

#[derive(Debug)]
pub struct Fetched {
  fetched: Vec<Dir>,
}

impl Fetched {
  pub fn print(&self) {
    println!("");
    if self.fetched.is_empty() {
      println!("Nothing to fetch.")
    } else {
      println!("Remote changes fetched for:\n");
      for fetched in &self.fetched {
        println!("  - {}", fetched.name);
      }
    }
  }
}

pub fn run_fetch(repos: Repos) {
  let fetched = execute_fetch(repos).unwrap();
  // dbg!(fetched);
  fetched.print();
}

pub fn execute_fetch(repos: Repos) -> Result<Fetched, FetchError> {
  let mut result: Vec<Dir> = Vec::new();
  for dir in repos.repos {
    let fetched = Command::new("git")
      .arg("fetch")
      .current_dir(&dir.path)
      .output()
      .map_err(|e| FetchError {
        error: e.to_string(),
      })?;

    let fetched = String::from_utf8_lossy(&fetched.stderr).to_string();
    if !fetched.is_empty() {
      result.push(dir);
    }
  }

  Ok(Fetched { fetched: result })
}
