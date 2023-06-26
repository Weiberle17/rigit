pub mod status;

use rigit::{run_requested_command, Args};

fn main() {
  let args: Args = Args::parse_args();

  run_requested_command(&args);
}
