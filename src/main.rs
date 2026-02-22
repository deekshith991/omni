// src/main.rs

mod cli;
mod commands;
mod package_manager;
mod utils;

use crate::cli::{Cli, Commands};
use clap::Parser;
use colorize::*;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            let _ = commands::init::run();
        }

        Commands::Install {
            package,
            general,
            global,
            comment,
        } => {
            commands::install::run(package, general, global, comment);
        }

        Commands::List => {
            let _ = commands::list::run();
        }

        Commands::Remove => {
            let _ = commands::remove::run();
        }
    }

    println!("{} Process completed successfully!\n", "[+]".green(),);
}
