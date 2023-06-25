use std::process::{Command, Output};

use rigit::check_output;

fn main() {
  let output: Output = Command::new("git").arg("status").output().unwrap();

  assert!(output.status.success());

  let output = String::from_utf8_lossy(&output.stdout);

  println!("{:?}", check_output(&output));
}
