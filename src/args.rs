use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Simple program to process CSV files
#[derive(Parser)]
#[clap(
    author = "Tito Titon",
    version,
    about = "A Very simple CSV processing app."
)]
pub struct Cli {
    /// Path to file which will be processed by the app
    pub path: PathBuf,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Delete n rows in the beginning of the file
    DeleteRows {
        /// Number of rows to delete
        #[arg(short, long)]
        n: u8,
    },
}
