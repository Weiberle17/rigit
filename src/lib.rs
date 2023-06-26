mod status;

use std::env;

pub struct Args {
  pub command: String,
  pub path: String,
}

impl Args {
  pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let command = args[1].to_owned();
    let path = args[2].to_owned();

    Args { command, path }
  }
}

// if args[1] == "status" {
//   let parent_dir = StatusParentDir::build(&args[2]).unwrap_or_else(|err| {
//     eprintln!("Problem parsing arguments: {err}");
//     process::exit(1);
//   });
// }
