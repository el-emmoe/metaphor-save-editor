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
        bail!("Incorrect number of arguments.")
    }

    let mut save_file = File::open(&args[1]).context("reading save file from argument")?;

    let magic = [0x16, 0x00, 0x00, 0x00];
    let mut save_buf = [0; 4];

    save_file
        .read_exact(&mut save_buf)
        .context("read first bytes from save")?;

    if save_buf != magic {
        bail!("Magic: Are you sure this is a save file?");
    }

    let mut save = Vec::from(save_buf);

    save_file
        .read_to_end(&mut save)
        .context("reading rest of the file")?;

    let general: StatsGeneral = StatsGeneral {
        name: String::from_utf8(save[NAME..NAME + 0x40].to_vec())
            .context("reading bytes to string")?
            .replace('\0', ""),
        mag: u32::from_le_bytes([save[MAG], save[MAG + 0x1], save[MAG + 0x2], save[MAG + 0x3]]),
        money: u32::from_le_bytes([
            save[MONEY],
            save[MONEY + 0x1],
            save[MONEY + 0x2],
            save[MONEY + 0x3],
        ]),
        courage: u16::from_le_bytes([save[VIRTUES], save[VIRTUES + 0x1]]),
        wisdom: u16::from_le_bytes([save[VIRTUES + 0x2], save[VIRTUES + 0x3]]),
        tolerance: u16::from_le_bytes([save[VIRTUES + 0x4], save[VIRTUES + 0x5]]),
        eloquence: u16::from_le_bytes([save[VIRTUES + 0x6], save[VIRTUES + 0x7]]),
        imagination: u16::from_le_bytes([save[VIRTUES + 0x8], save[VIRTUES + 0x9]]),
    };

    let members: Vec<StatsParty> = {
        let mut members = Vec::new();

        for member in PARTY {
            members.push(StatsParty {
                currhp: u32::from_le_bytes([
                    save[member],
                    save[member + 0x1],
                    save[member + 0x2],
                    save[member + 0x3],
                ]),
                currmp: u32::from_le_bytes([
                    save[member + 0x4],
                    save[member + 0x5],
                    save[member + 0x6],
                    save[member + 0x7],
                ]),
                totalhp: u32::from_le_bytes([
                    save[member + 0x8],
                    save[member + 0x9],
                    save[member + 0xA],
                    save[member + 0xB],
                ]),
                totalmp: u32::from_le_bytes([
                    save[member + 0xC],
                    save[member + 0xD],
                    save[member + 0xE],
                    save[member + 0xF],
                ]),
                lvl: u32::from_le_bytes([
                    save[member + 0x14],
                    save[member + 0x15],
                    save[member + 0x16],
                    save[member + 0x17],
                ]),
                exp: u32::from_le_bytes([
                    save[member + 0x18],
                    save[member + 0x19],
                    save[member + 0x1A],
                    save[member + 0x1B],
                ]),
                strength: save[member + 0x1E],
                magic: save[member + 0x1F],
                endurance: save[member + 0x20],
                agility: save[member + 0x21],
                luck: save[member + 0x22],
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
