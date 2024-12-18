use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::errors::AbootCrafterError;
use crate::types::{BootImgHeader, BOOT_MAGIC};

pub fn read_header(file: &mut File) -> Result<BootImgHeader, AbootCrafterError> {
    file.seek(SeekFrom::Start(0))?;

    let mut header = BootImgHeader::default();

    file.read_exact(&mut header.magic)?;
    if header.magic != *BOOT_MAGIC {
        return Err(AbootCrafterError::InvalidMagicBytes);
    }

    header.kernel_size = file.read_u32::<LittleEndian>()?;
    header.kernel_addr = file.read_u32::<LittleEndian>()?;
    header.ramdisk_size = file.read_u32::<LittleEndian>()?;
    header.ramdisk_addr = file.read_u32::<LittleEndian>()?;
    header.second_size = file.read_u32::<LittleEndian>()?;
    header.second_addr = file.read_u32::<LittleEndian>()?;
    header.tags_addr = file.read_u32::<LittleEndian>()?;
    header.page_size = file.read_u32::<LittleEndian>()?;

    file.read_u32::<LittleEndian>()?;
    file.read_u32::<LittleEndian>()?;

    file.read_exact(&mut header.name)?;
    file.read_exact(&mut header.cmdline)?;

    for id in &mut header.id {
        *id = file.read_u32::<LittleEndian>()?;
    }

    Ok(header)
}

pub fn write_header(file: &mut File, header: &BootImgHeader) -> Result<(), AbootCrafterError> {
    file.seek(SeekFrom::Start(0))?;

    file.write_all(&header.magic)?;
    file.write_u32::<LittleEndian>(header.kernel_size)?;
    file.write_u32::<LittleEndian>(header.kernel_addr)?;
    file.write_u32::<LittleEndian>(header.ramdisk_size)?;
    file.write_u32::<LittleEndian>(header.ramdisk_addr)?;
    file.write_u32::<LittleEndian>(header.second_size)?;
    file.write_u32::<LittleEndian>(header.second_addr)?;
    file.write_u32::<LittleEndian>(header.tags_addr)?;
    file.write_u32::<LittleEndian>(header.page_size)?;
    file.write_u32::<LittleEndian>(0)?;
    file.write_u32::<LittleEndian>(0)?;
    file.write_all(&header.name)?;
    file.write_all(&header.cmdline)?;

    for id in &header.id {
        file.write_u32::<LittleEndian>(*id)?;
    }

    Ok(())
}
