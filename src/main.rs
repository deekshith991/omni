// src/main.rs

use clap::{Parser, Subcommand};
use colorize::*;

mod install;

#[derive(Parser)]
#[command(name = "omni")]
#[command(about = "One Manager, No matter the Infrastructure", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package (simulated)
    Install { package: String },
    /// Show package info
    Info,
}

fn main() {
    println!(
        "\n{}",
        "OMNI — One Manager, No matter the Infrastructure"
            .green()
            .bold()
    );

    basic_data();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Install { package }) => {
            install::install_package(package);
        }
        Some(Commands::Info) => {
            println!("\nbasic info");
            basic_data();
        }
        None => {
            // Nothing else to do; banner and basic data already printed
        }
    }
}

fn basic_data() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    println!("{} {}", name.blue().bold(), version.blue().bold());
    println!("Author(s): {}\n", authors);
}
