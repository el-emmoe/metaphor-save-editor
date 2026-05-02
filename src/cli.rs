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

        /// amount of courage to set (max at 240)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=240))]
        courage: Option<u8>,

        /// amount of wisdom to set (max at 280)
        #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=280))]
        wisdom: Option<u16>,

        /// amount of tolerance to set (max at 210)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=210))]
        tolerance: Option<u8>,

        /// amount of eloquence to set (max at 170)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=170))]
        eloquence: Option<u8>,

        /// amount of imagination to set (max at 280)
        #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=280))]
        imagination: Option<u16>,
    },
    /// Edit party stats like HP, MP, level
    Party {
        /// party member starting from 1
        /// (Will is 1, Strohl 2 ...)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=8))]
        character: u8,

        /// amount of HP to set
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=999))]
        hp: Option<u16>,

        /// amount of MP to set
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=999))]
        mp: Option<u16>,

        /// level to set
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=999))]
        lvl: Option<u16>,

        /// amount of exp to set
        #[arg(long)]
        exp: Option<u32>,

        /// amount of strength to set
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=99))]
        strength: Option<u8>,

        /// amount of magic to set
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=99))]
        magic: Option<u8>,

        /// amount of endurance to set
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=99))]
        endurance: Option<u8>,

        /// amount of agility to set
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=99))]
        agility: Option<u8>,

        /// amount of luck to set
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=99))]
        luck: Option<u8>,
    },
}
