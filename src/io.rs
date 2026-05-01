use std::io::{BufWriter, Write};
use std::{fs::File, io::Seek};

use anyhow::{Context, Result};

pub(crate) fn read_u32(save: &[u8], offset: usize) -> Result<u32> {
    Ok(u32::from_le_bytes(
        save[offset..offset + 4]
            .try_into()
            .context("convert slice to vec for reading u32")?,
    ))
}
pub(crate) fn read_u16(save: &[u8], offset: usize) -> Result<u16> {
    Ok(u16::from_le_bytes(
        save[offset..offset + 2]
            .try_into()
            .context("convert slice to vec for reading u16")?,
    ))
}

pub(crate) fn write_u32(save: &mut BufWriter<File>, data: u32, offset: usize) -> Result<u32> {
    let bytes_to_write = data.to_le_bytes();
    save.seek(std::io::SeekFrom::Start(offset as u64))
        .context("seek given offset in vec buffer")?;
    save.write_all(&bytes_to_write)
        .context("write given data into save")?;
    Ok(data)
}
pub(crate) fn write_u16(save: &mut BufWriter<File>, data: u16, offset: usize) -> Result<u16> {
    let bytes_to_write = data.to_le_bytes();
    save.seek(std::io::SeekFrom::Start(offset as u64))
        .context("seek given offset in vec buffer")?;
    save.write_all(&bytes_to_write)
        .context("write given data into save")?;
    Ok(data)
}
