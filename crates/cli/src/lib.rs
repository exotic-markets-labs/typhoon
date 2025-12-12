pub mod new;

use {
    clap::{Parser, Subcommand},
    std::path::PathBuf,
};

#[derive(Debug, Parser, PartialEq)]
#[command(name = "typhoon")]
#[command(about = "Typhoon CLI - Solana Sealevel Framework", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum Commands {
    /// Create a new Typhoon project
    New {
        /// Project name
        name: String,
        /// Project directory path
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Force overwrite existing files
        #[arg(short, long, default_value = "false")]
        force: bool,
        /// Typhoon workspace path to use instead of the crate version
        #[arg(long)]
        typhoon_path: Option<PathBuf>,
    },
}
