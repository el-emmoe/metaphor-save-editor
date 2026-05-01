use anyhow::Result;
use clap::Parser;

mod cli;
mod io;
mod offsets;
mod process;
mod stats;

use cli::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    process::process_save(&args)?;

    Ok(())
}
