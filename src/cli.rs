use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to save file
    #[arg(short, long)]
    pub(crate) file: PathBuf,

    /// Show all stats (default)
    #[arg(short, long, default_value = "true")]
    pub(crate) show: bool,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Edit general stats like money, mag or virtues
    Stats {
        /// amount of mag to set
        #[arg(long)]
        mag: Option<u32>,

        /// amount of money to set
        #[arg(long)]
        money: Option<u32>,

        /// amount of courage to set
        #[arg(short, long)]
        courage: Option<u16>,

        /// amount of wisdom to set
        #[arg(short, long)]
        wisdom: Option<u16>,

        /// amount of tolerance to set
        #[arg(short, long)]
        tolerance: Option<u16>,

        /// amount of eloquence to set
        #[arg(short, long)]
        eloquence: Option<u16>,

        /// amount of imagination to set
        #[arg(short, long)]
        imagination: Option<u16>,
    },
    /// Edit party stats like HP, MP, level
    Party,
}
