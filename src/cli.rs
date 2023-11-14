//!
//! # CLI argument handler
//!
//! This module handles the CLI arguments using clap

use clap::{Parser, Subcommand};

/// Struct that implements clap to setup CLI arguments
#[derive(Parser, Debug)]
#[command(name = "rigit")]
#[command(author = "Weiberle17 <maxweiberle63@gmail.com>")]
#[command(version)]
#[command(about = "A CLI tool which lets you perform a git action on multiple repositories in one directory", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

/// Enum that includes all the possible CLI arguments and their subarguments
#[derive(Subcommand, Debug)]
pub enum Command {
  /// Run git status on all repos in the following directory
  Status {
    path: String,
    #[clap(short, long)]
    verbose: bool,
  },
  /// Run git fetch on all repos in the following directory
  Fetch { path: String },
}
