// src/cli.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "omni")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,

    Install {
        package: String,

        #[arg(long, conflicts_with = "global")]
        local: bool,

        #[arg(long, conflicts_with = "local")]
        global: bool,
    },
}
