use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "omni",
    version,
    about = "Omni is a lightweight package manager",
    long_about = None
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
        local: bool,

        /// Install the package globally
        #[arg(long, conflicts_with = "local")]
        global: bool,
    },
}
