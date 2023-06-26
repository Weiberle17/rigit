mod status;

use status::{printing, ParentDir};
use std::{env, process};

fn main() {
  let args: Vec<String> = env::args().collect();

  let parent_dir = ParentDir::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  printing(parent_dir);
}
