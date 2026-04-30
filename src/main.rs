use std::{fs::File, io::Read};

use anyhow::{Context, Result, bail};

const NAME: usize = 0x5c;
const MAG: usize = 0x9d8;
const MONEY: usize = 0x23820;
const VIRTUES: usize = 0x246b0;
const PARTY: [usize; 8] = [
    0x248c, /* Will */
    0x3b34, /* Strohl */
    0x51dc, /* Hulkenberg */
    0x6884, /* Heismay */
    0x7f2c, /* ???? */
    0x95d4, /* ???? */
    0xc324, /* ???? */
    0xf074, /* ???? */
];
const MAGIC: [u8; 4] = [0x16, 0x00, 0x00, 0x00];
const EXPECTED_LEN_MIN: usize = 1500000;

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
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Incorrect number of arguments.\nUsage: r <path to file>")
    }

    let mut save_file = File::open(&args[1]).context("reading save file from argument")?;
    let mut save_buf = [0; 4];

    save_file
        .read_exact(&mut save_buf)
        .context("read first bytes from save")?;

    if save_buf != MAGIC {
        bail!("Magic: Are you sure this is a save file?");
    }

    let mut save = Vec::from(save_buf);

    save_file
        .read_to_end(&mut save)
        .context("reading rest of the file")?;

    if save.len() < EXPECTED_LEN_MIN {
        bail!("Save file seems invalid: too short.")
    }

    let general: StatsGeneral = StatsGeneral {
        name: String::from_utf8(save[NAME..NAME + 64].to_vec())
            .context("reading bytes to string")?
            .replace('\0', ""),
        mag: read_u32(&save, MAG).context("read bytes at MAG offset to u32")?,
        money: read_u32(&save, MONEY).context("read bytes at MONEY offset to u32")?,
        courage: read_u16(&save, VIRTUES).context("read bytes at COURAGE offset to u16")?,
        wisdom: read_u16(&save, VIRTUES + 2).context("read bytes at WISDOM offset to u16")?,
        tolerance: read_u16(&save, VIRTUES + 4).context("read bytes at TOLERANCE offset to u16")?,
        eloquence: read_u16(&save, VIRTUES + 6).context("read bytes at ELOQUENCE offset to u16")?,
        imagination: read_u16(&save, VIRTUES + 8).context("read bytes at IMAG offset to u16")?,
    };

    let members: Vec<StatsParty> = {
        let mut members = Vec::new();

        for member in PARTY {
            members.push(StatsParty {
                currhp: read_u32(&save, member).context("read bytes at CURRHP offset to u32")?,
                currmp: read_u32(&save, member + 4)
                    .context("read bytes at CURRMP offset to u32")?,
                totalhp: read_u32(&save, member + 8)
                    .context("read bytes at TOTALHP offset to u32")?,
                totalmp: read_u32(&save, member + 12)
                    .context("read bytes at TOTALMP offset to u32")?,
                lvl: read_u32(&save, member + 20).context("read bytes at LVL offset to u32")?,
                exp: read_u32(&save, member + 24).context("read bytes at EXP offset to u32")?,
                strength: save[member + 30],
                magic: save[member + 31],
                endurance: save[member + 32],
                agility: save[member + 33],
                luck: save[member + 34],
            });
        }
        members
    };

    println!("General: {:#?}", general);

    for (i, member) in members.iter().enumerate() {
        println!("Member {}: {:#?}", i + 1, member);
    }

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
