// src/main.rs

use clap::{Args, Parser, Subcommand};
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
    /// Install a package
    Install(InstallArgs),

    /// Show package info
    Info,
}

#[derive(Args)]
struct InstallArgs {
    /// Package name
    package: String,

    /// Install system-wide
    #[arg(long, conflicts_with = "general")]
    global: bool,

    /// Install for current user
    #[arg(long, conflicts_with = "global")]
    general: bool,
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

    match cli.command {
        Some(Commands::Install(args)) => {
            install::dispatch(args);
        }
        Some(Commands::Info) => {
            println!("\nbasic info");
            basic_data();
        }
        None => {}
    }
}

fn basic_data() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    println!("{} {}", name.blue().bold(), version.blue().bold());
    println!("Author(s): {}\n", authors);
}
