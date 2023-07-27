use clap::Parser;

#[derive(Default, Parser, Debug)]
#[command(name = "rigit")]
#[command(author = "Weiberle17 <maxweiberle63@gmail.com>")]
#[command(version)]
#[command(about = "Cli-tool which lets you perform a git action on multiple repositories", long_about = None)]
pub struct Cli {
  command: String,
  path: String,
}
