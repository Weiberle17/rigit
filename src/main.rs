mod cli;
mod status;
mod repos;

use clap::Parser;
use cli::Cli;
use repos::Repos;
use status::get_status;

fn main() {
  let args = Cli::parse();
  let repos = Repos::get_repos(args.path).unwrap();
  
  match args.command.as_str() {
    "status" => run_status(repos),
    _ => eprintln!("Not a supported command"),
  }
}
