// src/main.rs

mod cli;
mod commands;
mod utils;

use crate::cli::{Cli, Commands};
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

    println!("Process completed successfully!\n");
}
