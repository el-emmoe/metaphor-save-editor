use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use std::{
    fs::File,
    io::{BufWriter, Read, Seek, Write},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to save file
    #[arg(short, long)]
    file: PathBuf,

    /// Show all stats (default)
    #[arg(short, long, default_value = "true")]
    show: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
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

const NAME: usize = 0x5c;
const MAG: usize = 0x9d8;
const MONEY: usize = 0x23820;
const VIRTUES: usize = 0x246b0;
const PARTY: [usize; 8] = [
    0x248c, /* Will */
    0x3b34, /* Strohl */
    0x51dc, /* Hulkenberg */
    0x6884, /* Heismay */
    // not finished with the game myself yet l o l
    0x7f2c, /* ???? */
    0x95d4, /* ???? */
    0xc324, /* ???? */
    0xf074, /* ???? */
];
const MAGIC: [u8; 4] = [0x16, 0x00, 0x00, 0x00];
const EXPECTED_LEN_RANGE: (usize, usize) = (1500000, 3000000);

#[derive(Debug)]
#[allow(unused)]
struct StatsGeneral {
    name: String,
    mag: u32,
    money: u32,
    courage: u16,
    wisdom: u16,
    tolerance: u16,
    eloquence: u16,
    imagination: u16,
}

#[derive(Debug)]
#[allow(unused)]
struct StatsParty {
    currhp: u32,
    currmp: u32,
    totalhp: u32,
    totalmp: u32,
    lvl: u32,
    exp: u32,
    strength: u8,
    magic: u8,
    endurance: u8,
    agility: u8,
    luck: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

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

fn read_u32(save: &[u8], offset: usize) -> Result<u32> {
    Ok(u32::from_le_bytes(
        save[offset..offset + 4]
            .try_into()
            .context("convert slice to vec for reading u32")?,
    ))
}
fn read_u16(save: &[u8], offset: usize) -> Result<u16> {
    Ok(u16::from_le_bytes(
        save[offset..offset + 2]
            .try_into()
            .context("convert slice to vec for reading u16")?,
    ))
}

fn write_u32(save: &mut BufWriter<File>, data: u32, offset: usize) -> Result<u32> {
    let bytes_to_write = data.to_le_bytes();
    save.seek(std::io::SeekFrom::Start(offset as u64))
        .context("seek given offset in vec buffer")?;
    save.write_all(&bytes_to_write)
        .context("write given data into save")?;
    Ok(data)
}
fn write_u16(save: &mut BufWriter<File>, data: u16, offset: usize) -> Result<u16> {
    let bytes_to_write = data.to_le_bytes();
    save.seek(std::io::SeekFrom::Start(offset as u64))
        .context("seek given offset in vec buffer")?;
    save.write_all(&bytes_to_write)
        .context("write given data into save")?;
    Ok(data)
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
