use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

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

    /// Encoding of the source file
    pub encoding: Option<KnownEncodings>,

    /// app commands
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
    /// Replace n/a values with provided string/char
    ReplaceNA {
        /// cell value to replace n/a
        #[arg(short, long)]
        val: String,
    },
    /// Change format of dates in the source file to the valid one
    ChangeDatesFormat {
        /// specify format used in source file in %m.%d.%y like format
        #[arg(short, long)]
        format: String,
        #[arg(short, long)]
        /// specify number of the row with dates data
        #[arg(default_value_t = 0)]
        dates_row_number: usize,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum KnownEncodings {
    /// utf 16 encoding
    Utf16,
    /// utf 8 encoding
    Utf8,
}
