//!
//! # rigit
//!
//! A CLI tool which let's you perform a git action on multiple repositories in one directory

mod cli;
mod fetch;
mod repos;
mod status;

use clap::Parser;
use cli::{Cli, Command};
use fetch::run_fetch;
use repos::Repos;
use status::run_status;
use std::path::Path;

fn main() {
  let args = Cli::parse();

  match args.command {
    Command::Status { path, verbose } => {
      if !Path::new(&path).exists() {
        panic!("\n The given path is not available. Maybe you are missing a trailing '/'.\n Use 'rigit -h' or 'rigit --help'.\n");
      }
      let repos = Repos::get_repos(path).unwrap();
      run_status(repos, verbose);
    }
    Command::Fetch { path } => {
      if !Path::new(&path).exists() {
        panic!("\n The given path is not available. Maybe you are missing a trailing '/'.\n Use 'rigit -h' or 'rigit --help'.\n");
      }
      let repos = Repos::get_repos(path).unwrap();
      run_fetch(repos);
    }
  }
}
