mod status;

use rigit::Args;
use status::StatusParentDir;
use std::process;

fn main() {
  let args: Args = Args::parse_args();

  if args.command == "status" {
    let parent_dir = StatusParentDir::build(&args.path).unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {err}");
      process::exit(1);
    });
    parent_dir.printing();
  }
}
