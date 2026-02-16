// src/main.rs

mod cli;
mod commands;

use self::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            commands::init::run();
        }

        Commands::Install {
            package,
            local,
            global,
        } => {
            commands::install::run(package, local, global);
        }
    }
}
