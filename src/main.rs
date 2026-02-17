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
            let _ = commands::init::run();
        }

        Commands::Install {
            package,
            local,
            global,
            comment,
        } => {
            commands::install::run(package, local, global, comment);
        }
    }

    println!("Process completed successfully!\n");
}
