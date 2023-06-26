pub mod pull;
pub mod status;

use status::StatusParentDir;
use std::env;
use pull::PullParentDir;

pub struct Args {
  pub command: String,
  pub path: String,
}

impl Args {
  pub fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
      return Err("Usage: rigit [command] [directory]".to_string());
    }
    let command = args[1].to_owned();
    let path = args[2].to_owned();

    Ok(Args { command, path })
  }
}

pub fn run_requested_command(args: &Args) -> Result<(), String> {
  match args.command.as_str() {
    "status" => {
      let parent_dir = StatusParentDir::build(&args.path);
      parent_dir.printing();
    }
    "pull" => {
      let parent_dir = PullParentDir::build(&args.path);
      parent_dir.printing();
    }
    _ => {
      let error = format!(
        "Usable commands are: [status, pull], you used '{}'",
        args.command
      );
      return Err(error);
    }
  }
  
  Ok(())
}
