use std::{env, path::PathBuf, process};

use rigit::{get_status, ParentDir};

fn main() {
  let args: Vec<String> = env::args().collect();

  let parent_dir = ParentDir::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  for dir in parent_dir.child_directories {
    println!("{}", dir.0);
    println!("{:?}", get_status(&dir.1));
  }
}
