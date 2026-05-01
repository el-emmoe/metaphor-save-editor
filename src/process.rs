use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result, bail};

use crate::{
    cli::{Args, Commands},
    io::{read_u16, read_u32, write_u16, write_u32},
    offsets::*,
    stats::{StatsGeneral, StatsParty},
};

const MAGIC: [u8; 4] = [0x16, 0x00, 0x00, 0x00];
const EXPECTED_LEN_RANGE: (usize, usize) = (1500000, 3000000);

pub fn process_save(args: &Args) -> Result<()> {
    let mut save_file = File::options()
        .read(true)
        .write(true)
        .open(&args.file)
        .context("reading save file from argument")?;
    let mut init_save_buf = [0; 4];

    save_file
        .read_exact(&mut init_save_buf)
        .context("read first bytes from save")?;

    if init_save_buf != MAGIC {
        bail!("Magic: Are you sure this is a save file?");
    }

    let mut save = Vec::from(init_save_buf);

    save_file
        .read_to_end(&mut save)
        .context("reading rest of the file")?;

    if !(EXPECTED_LEN_RANGE.0..=EXPECTED_LEN_RANGE.1).contains(&save.len()) {
        bail!("Size of file outside of expected range: {}", save.len());
    }

    let backup = PathBuf::from(format!("{}.bak", &args.file.display()));
    if !backup.exists() {
        std::fs::copy(&args.file, backup).context("creating backup of save file")?;
    }

    let mut file = BufWriter::new(save_file);

    match &args.command {
        Some(Commands::Stats {
            mag,
            money,
            courage,
            wisdom,
            tolerance,
            eloquence,
            imagination,
        }) => {
            if let Some(mag) = mag {
                write_u32(&mut file, *mag, MAG)?;
            }
            if let Some(money) = money {
                write_u32(&mut file, *money, MONEY)?;
            }
            if let Some(courage) = courage {
                write_u16(&mut file, *courage, VIRTUES)?;
            }
            if let Some(wisdom) = wisdom {
                write_u16(&mut file, *wisdom, VIRTUES + 2)?;
            }
            if let Some(tolerance) = tolerance {
                write_u16(&mut file, *tolerance, VIRTUES + 4)?;
            }
            if let Some(eloquence) = eloquence {
                write_u16(&mut file, *eloquence, VIRTUES + 6)?;
            }
            if let Some(imagination) = imagination {
                write_u16(&mut file, *imagination, VIRTUES + 8)?;
            }
        }
        Some(Commands::Party) => {}
        None => {}
    }

    file.flush().context("flush bufwriter output")?;

    print_stats(&save, args.show)?;

    Ok(())
}

fn print_stats(save: &[u8], show: bool) -> Result<()> {
    let general: StatsGeneral = StatsGeneral {
        name: String::from_utf8(save[NAME..NAME + 64].to_vec())
            .context("reading bytes to string")?
            .replace('\0', ""),
        mag: read_u32(save, MAG).context("read bytes at MAG offset to u32")?,
        money: read_u32(save, MONEY).context("read bytes at MONEY offset to u32")?,
        courage: read_u16(save, VIRTUES).context("read bytes at COURAGE offset to u16")?,
        wisdom: read_u16(save, VIRTUES + 2).context("read bytes at WISDOM offset to u16")?,
        tolerance: read_u16(save, VIRTUES + 4).context("read bytes at TOLERANCE offset to u16")?,
        eloquence: read_u16(save, VIRTUES + 6).context("read bytes at ELOQUENCE offset to u16")?,
        imagination: read_u16(save, VIRTUES + 8).context("read bytes at IMAG offset to u16")?,
    };

    let _members: Vec<StatsParty> = {
        let mut members = Vec::new();

        for member in PARTY {
            members.push(StatsParty {
                currhp: read_u32(save, member).context("read bytes at CURRHP offset to u32")?,
                currmp: read_u32(save, member + 4).context("read bytes at CURRMP offset to u32")?,
                totalhp: read_u32(save, member + 8)
                    .context("read bytes at TOTALHP offset to u32")?,
                totalmp: read_u32(save, member + 12)
                    .context("read bytes at TOTALMP offset to u32")?,
                lvl: read_u32(save, member + 20).context("read bytes at LVL offset to u32")?,
                exp: read_u32(save, member + 24).context("read bytes at EXP offset to u32")?,
                strength: save[member + 30],
                magic: save[member + 31],
                endurance: save[member + 32],
                agility: save[member + 33],
                luck: save[member + 34],
            });
        }
        members
    };

    if show {
        println!("General: {:#?}", general);

        // for (i, member) in members.iter().enumerate() {
        //     println!("Member {}: {:#?}", i + 1, member);
        // }
    }

    Ok(())
}
