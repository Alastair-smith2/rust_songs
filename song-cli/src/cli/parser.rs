use clap::Parser;

use crate::cli::commands::Command;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// The Command to execute
    #[clap(subcommand)]
    pub command: Command,

    #[clap(short, long)]
    pub csv: String,
}
