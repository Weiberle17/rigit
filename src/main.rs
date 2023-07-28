mod cli;
mod repos;
mod status;

use clap::Parser;
use cli::{Cli, Command};
use repos::Repos;
use status::run_status;

fn main() {
  let args = Cli::parse();

  match args.command {
    Command::Status { path, verbose } => {
      let repos = Repos::get_repos(path).unwrap();
      run_status(repos, verbose);
    }
  }
}
