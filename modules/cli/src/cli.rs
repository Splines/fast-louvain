// TODO: modularize the CLI and add human-readable error messages
// see https://rust-cli.github.io/book/tutorial/errors.html
// TODO: add logging to application

use clap::{Parser, Subcommand};

use crate::{
    cli_hierarchy::{self, HierarchyArgs},
    cli_louvain::{self, LouvainArgs},
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Community(LouvainArgs),
    Hierarchy(HierarchyArgs),
}

pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Community(args) => {
            cli_louvain::run(args);
        }
        Commands::Hierarchy(args) => {
            cli_hierarchy::run(args);
        }
    }
}
