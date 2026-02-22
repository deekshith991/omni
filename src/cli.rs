// src/cli.rs

use clap::{ColorChoice, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "omni",
    version,
    about = "\x1b[32mOmni is a lightweight package manager\x1b[0m",
    long_about = None,
    color = ColorChoice::Auto
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Initialize a new omni project")]
    Init,

    #[command(about = "Install a package")]
    Install {
        /// Name of the package to install
        package: String,

        /// Install the package locally
        #[arg(long, conflicts_with = "global")]
        general: bool,

        /// Install the package globally
        #[arg(long, conflicts_with = "general")]
        global: bool,

        /// Optional comment for this installation
        #[arg(long, value_name = "TEXT")]
        comment: Option<String>,
    },

    // list the packages in toml
    #[command(about = "List installed packages")]
    List,

    // Remove packages from the toml
    // Remove packages from the toml
    #[command(about = "Remove installed package")]
    Remove {
        /// remove package
        package: String,

        #[arg(long, conflicts_with_all = ["pm", "general", "global"])]
        all: bool,

        #[arg(long, conflicts_with_all = ["all", "general", "global"])]
        pm: bool,

        #[arg(long, conflicts_with_all = ["all", "pm", "global"])]
        general: bool,

        #[arg(long, conflicts_with_all = ["all", "pm", "general"])]
        global: bool,
    },
}
