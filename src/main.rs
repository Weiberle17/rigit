use std::process;

use rigit::{run_requested_command, Args};

fn main() {
  let args: Args = Args::parse_args().unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  if let Err(e) = run_requested_command(&args) {
    eprintln!("Application error: {e}");
    process::exit(1);
  };
}
