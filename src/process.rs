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
const EXPECTED_LEN_RANGE: (usize, usize) = (1000000, 3000000);

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
                write_u16(&mut file, *courage as u16, VIRTUES)?;
            }
            if let Some(wisdom) = wisdom {
                write_u16(&mut file, *wisdom, VIRTUES + 2)?;
            }
            if let Some(tolerance) = tolerance {
                write_u16(&mut file, *tolerance as u16, VIRTUES + 4)?;
            }
            if let Some(eloquence) = eloquence {
                write_u16(&mut file, *eloquence as u16, VIRTUES + 6)?;
            }
            if let Some(imagination) = imagination {
                write_u16(&mut file, *imagination, VIRTUES + 8)?;
            }
        }
        Some(Commands::Party {
            character,
            hp,
            mp,
            lvl,
            exp,
            strength,
            magic,
            endurance,
            agility,
            luck,
        }) => {
            let party_char = PARTY[(character - 1) as usize];

            if let Some(hp) = hp {
                write_u16(&mut file, *hp, party_char + HP)?;
                write_u16(&mut file, *hp, party_char + HP + 8)?;
            }
            if let Some(mp) = mp {
                write_u16(&mut file, *mp, party_char + MP)?;
                write_u16(&mut file, *mp, party_char + MP + 8)?;
            }
            if let Some(lvl) = lvl {
                write_u16(&mut file, *lvl, party_char + LVL)?;
            }
            if let Some(exp) = exp {
                write_u32(&mut file, *exp, party_char + EXP)?;
            }
            if let Some(strength) = strength {
                write_u16(&mut file, *strength as u16, party_char + STATS)?;
            }
            if let Some(magic) = magic {
                write_u16(&mut file, *magic as u16, party_char + STATS + 1)?;
            }
            if let Some(endurance) = endurance {
                write_u16(&mut file, *endurance as u16, party_char + STATS + 2)?;
            }
            if let Some(agility) = agility {
                write_u16(&mut file, *agility as u16, party_char + STATS + 3)?;
            }
            if let Some(luck) = luck {
                write_u16(&mut file, *luck as u16, party_char + STATS + 4)?;
            }
        }
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

    let characters: Vec<StatsParty> = {
        let mut characters = Vec::new();

        for character in PARTY {
            characters.push(StatsParty {
                currhp: read_u32(save, character + HP)
                    .context("read bytes at CURRHP offset to u32")?,
                currmp: read_u32(save, character + MP)
                    .context("read bytes at CURRMP offset to u32")?,
                totalhp: read_u32(save, character + HP + 8)
                    .context("read bytes at TOTALHP offset to u32")?,
                totalmp: read_u32(save, character + MP + 8)
                    .context("read bytes at TOTALMP offset to u32")?,
                lvl: read_u32(save, character + LVL).context("read bytes at LVL offset to u32")?,
                exp: read_u32(save, character + EXP).context("read bytes at EXP offset to u32")?,
                strength: save[character + STATS],
                magic: save[character + STATS + 1],
                endurance: save[character + STATS + 2],
                agility: save[character + STATS + 3],
                luck: save[character + STATS + 4],
            });
        }
        characters
    };

    if show {
        println!("General: {:#?}", general);

        for (i, c) in characters.iter().enumerate() {
            println!("Character {}: {:#?}", i + 1, c);
        }
    }

    Ok(())
}
