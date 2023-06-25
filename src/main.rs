use std::{
  env,
  path::PathBuf,
  process,
};

use rigit::{status, ParentDir};

fn main() {
  let args: Vec<String> = env::args().collect();

  let parent_dir = ParentDir::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  dbg!(&parent_dir);

  for dir in parent_dir.child_directories {
    let dir = format!("{}{}", parent_dir.path, dir);
    if PathBuf::from(&dir).is_dir() {
      dbg!(status(dir));
    }
  }
}
