use std::{collections::HashMap, io::Error, path::PathBuf, process::Command};

use crate::Dir;

#[derive(Debug)]
pub struct PullParentDir {
  pub path: String,
  pub pullable_dirs: Vec<Dir>,
}

impl PullParentDir {
  pub fn build(arg: &str) -> PullParentDir {
    let path = arg.to_owned();
    let child_directories = get_child_directories(&path).unwrap();
    let mut pullable_dirs: Vec<Dir> = Vec::new();
    for dir in child_directories {
      pullable_dirs.push(Dir {
        name: dir.0,
        path: dir.1,
      });
    }

    PullParentDir {
      path,
      pullable_dirs,
    }
  }

  pub fn printing(self: &Self) {
    println!("{self:?}");
  }
}

fn get_child_directories(parent_path: &str) -> Result<HashMap<String, String>, Error> {
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

fn check_repo(dir: &str) -> bool {
  let dir = format!("{}/.git", dir);
  if PathBuf::from(&dir).is_dir() {
    true
  } else {
    false
  }
}
