use std::{env, process};

use rigit::{ParentDir, printing};

fn main() {
  let args: Vec<String> = env::args().collect();

  let parent_dir = ParentDir::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  printing(parent_dir);
}
