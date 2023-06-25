use std::{io::Error, process::Command};

#[derive(Debug)]
pub struct ParentDir {
  pub path: String,
  pub command: String,
  pub child_directories: Vec<String>,
}

impl ParentDir {
  pub fn build(args: &[String]) -> Result<ParentDir, &'static str> {
    if args.len() != 3 {
      return Err("Not the right amount of arguments, expected 2");
    }
    let command = args[1].to_owned();
    let path = args[2].to_owned();
    let child_directories = get_child_directories(&path).unwrap();

    Ok(ParentDir {
      command,
      path,
      child_directories,
    })
  }
}

pub fn get_child_directories(parent_path: &str) -> Result<Vec<String>, Error> {
  let child_directories = Command::new("ls")
    .current_dir(parent_path)
    .output()
    .unwrap();

  let child_directories = String::from_utf8_lossy(&child_directories.stdout);

  let mut result: Vec<String> = Vec::new();

  for dir in child_directories.lines() {
    result.push(dir.to_string());
  }

  Ok(result)
}

pub fn status(dir: String) -> Result<Vec<String>, &'static str> {
  let mut output: Vec<String> = Vec::new();
  let dir = Command::new("git")
    .arg("status")
    .current_dir(dir)
    .output()
    .unwrap();
  assert!(dir.status.success());
  output.push(String::from_utf8_lossy(&dir.stdout).to_string());

  Ok(output)
}

pub fn check_status<'a>(contents: &'a str) -> Result<Vec<&'a str>, String> {
  let mut results = Vec::new();

  if !contents.contains("up to date") {
    results.push("Local repository is not synchronized with the remote repository.");
  }
  if contents.contains("modified") {
    results.push("You have uncommited changes in your local repository.");
  }
  if contents.contains("untracked") || contents.contains("new file") {
    results.push("You have untracked files in your repository")
  }
  if results.is_empty() {
    return Err("The repository is clean!".to_string());
  }

  Ok(results)
}

#[cfg(test)]
mod tests {
  use super::*;
}
