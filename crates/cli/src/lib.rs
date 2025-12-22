pub mod add;
pub mod new;
pub mod templates;
mod utils;

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
        /// Program name
        #[arg(long)]
        program: Option<String>,
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

    /// Add a program to the Typhoon workspace or an instruction to a program
    Add {
        #[command(subcommand)]
        subcommand: AddSubcommand,
    },
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum AddSubcommand {
    /// Add a program to the Typhoon workspace  
    Program {
        /// Project directory path
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Program name
        name: String,
    },

    /// Add a handler to a program
    Handler {
        /// Project directory path
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Program name
        program: String,
        /// Handler name
        name: String,
    },
}
