pub mod pull;
pub mod status;

use status::StatusParentDir;
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

pub fn run_requested_command(args: &Args) {
  if args.command == "status" {
    let parent_dir = StatusParentDir::build(&args.path);
    parent_dir.printing();
  } else {
    panic!("Usable commands are: 'status', you used {}", args.command);
  }
}
