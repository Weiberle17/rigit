use colored::Colorize;
use std::{collections::HashMap, io::Error, path::PathBuf, process::Command};

#[derive(Debug)]
pub struct ParentDir {
  pub path: String,
  pub command: String,
  pub status: Vec<Status>,
}

#[derive(Debug)]
pub struct Status {
  pub directory: Dir,
  pub status: Result<String, String>,
}

#[derive(Debug)]
pub struct Dir {
  pub name: String,
  pub path: String,
}

impl ParentDir {
  pub fn build(args: &[String]) -> Result<ParentDir, &'static str> {
    if args.len() != 3 {
      return Err("Not the right amount of arguments, expected 2");
    }
    let command = args[1].to_owned();
    let path = args[2].to_owned();
    let child_directories = get_child_directories(&path).unwrap();
    let mut status: Vec<Status> = Vec::new();
    for dir in child_directories {
      status.push(Status {
        status: get_status(&dir.1),
        directory: Dir {
          name: dir.0,
          path: dir.1,
        },
      });
    }

    status.sort_by_key(|item| match item.status {
      Ok(_) => 0,
      Err(_) => 1,
    });

    Ok(ParentDir {
      path,
      command,
      status,
    })
  }
}

pub fn get_child_directories(parent_path: &str) -> Result<HashMap<String, String>, Error> {
  let child_directories = Command::new("ls")
    .current_dir(parent_path)
    .output()
    .unwrap();

  let child_directories = String::from_utf8_lossy(&child_directories.stdout);

  let mut result: HashMap<String, String> = HashMap::new();

  for dir in child_directories.lines() {
    let repo = format!("{}{}", parent_path, dir);
    if check_repo(&repo) {
      result.insert(dir.to_string(), repo.to_string());
    }
  }

  Ok(result)
}

pub fn check_repo(dir: &str) -> bool {
  let dir = format!("{}/.git", dir);
  if PathBuf::from(&dir).is_dir() {
    true
  } else {
    false
  }
}

pub fn get_status(dir: &str) -> Result<String, String> {
  let dir = Command::new("git")
    .arg("status")
    .current_dir(dir)
    .output()
    .unwrap();
  assert!(dir.status.success());
  let content = String::from_utf8_lossy(&dir.stdout).to_string();

  match check_status(&content) {
    Ok(r) => return Ok(r),
    Err(e) => return Err(e),
  };
}

pub fn check_status<'a>(contents: &'a str) -> Result<String, String> {
  let mut results = Vec::new();

  if !contents.contains("up to date") {
    results.push("Local repository is not synchronized with the remote repository.\n".to_string());
  }
  if contents.contains("modified") {
    results.push("You have uncommited changes in your local repository.\n".to_string());
  }
  if contents.contains("untracked") || contents.contains("new file") {
    results.push("You have untracked files in your repository.\n".to_string())
  }
  if results.is_empty() {
    return Err("The repository is clean!".to_string());
  }

  let mut lines = String::new();
  for line in results {
    lines += &line;
  }

  Ok(lines)
}

pub fn printing(parent_dir: ParentDir) {
  for status in parent_dir.status {
    match status.status {
      Ok(r) => {
        println!("{:?}: {}", status.directory.name, " ".red());
        for line in r.lines() {
          println!("{:?}", line);
        }
        println!("");
      }
      Err(_) => {
        println!("{:?}: {}", status.directory.name, " ".green());
        println!("");
      }
    }
  }
}
