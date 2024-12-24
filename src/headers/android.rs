use std::{
    fs::{File, OpenOptions},
    io::{self, Seek, SeekFrom, Write},
    path::Path,
};

use super::fields::{
    AddressU32, AddressU64, AndroidBootMagic, Cmdline, CmdlineExtended, ExtraCmdline, Id, Name,
    OSVersion,
};
use binrw::{BinRead, BinWrite};

pub const PAGE_SIZE_V3: u32 = 4096;

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion0 {
    pub magic: AndroidBootMagic,
    pub kernel_size: u32,
    pub kernel_addr: AddressU32,
    pub ramdisk_size: u32,
    pub ramdisk_addr: AddressU32,
    pub second_size: u32,
    pub second_addr: AddressU32,
    pub tags_addr: AddressU32,
    pub page_size: u32,
    pub header_version: u32,
    pub os_version: OSVersion,
    pub name: Name,
    pub cmdline: Cmdline,
    pub id: Id,
    pub extra_cmdline: ExtraCmdline,
}

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion1 {
    pub magic: AndroidBootMagic,
    pub kernel_size: u32,
    pub kernel_addr: AddressU32,
    pub ramdisk_size: u32,
    pub ramdisk_addr: AddressU32,
    pub second_size: u32,
    pub second_addr: AddressU32,
    pub tags_addr: AddressU32,
    pub page_size: u32,
    pub header_version: u32,
    pub os_version: OSVersion,
    pub name: Name,
    pub cmdline: Cmdline,
    pub id: Id,
    pub extra_cmdline: ExtraCmdline,
    pub recovery_dtbo_size: u32,
    pub recovery_dtbo_offset: AddressU64,
    pub header_size: u32,
}

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion2 {
    pub magic: AndroidBootMagic,
    pub kernel_size: u32,
    pub kernel_addr: AddressU32,
    pub ramdisk_size: u32,
    pub ramdisk_addr: AddressU32,
    pub second_size: u32,
    pub second_addr: AddressU32,
    pub tags_addr: AddressU32,
    pub page_size: u32,
    pub header_version: u32,
    pub os_version: OSVersion,
    pub name: Name,
    pub cmdline: Cmdline,
    pub id: Id,
    pub extra_cmdline: ExtraCmdline,
    pub recovery_dtbo_size: u32,
    pub recovery_dtbo_offset: AddressU64,
    pub header_size: u32,
    pub dtb_size: u32,
    pub dtb_addr: AddressU64,
}

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion3 {
    pub magic: AndroidBootMagic,
    pub kernel_size: u32,
    pub ramdisk_size: u32,
    pub os_version: OSVersion,
    pub header_size: u32,
    pub reserved: [u32; 4],
    pub header_version: u32,
    pub cmdline: CmdlineExtended,
}

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion4 {
    pub magic: AndroidBootMagic,
    pub kernel_size: u32,
    pub ramdisk_size: u32,
    pub os_version: OSVersion,
    pub header_size: u32,
    pub reserved: [u32; 4],
    pub header_version: u32,
    pub cmdline: CmdlineExtended,
    pub signature_size: u32,
}

#[derive(Debug, Default)]
pub struct AndroidBootFile {
    pub header: AndroidHeader,
    pub version: u32,
    pub file: Option<File>,
}

#[derive(Debug)]
pub enum AndroidHeader {
    V0(AndroidHeaderVersion0),
    V1(AndroidHeaderVersion1),
    V2(AndroidHeaderVersion2),
    V3(AndroidHeaderVersion3),
    V4(AndroidHeaderVersion4),
}

impl Default for AndroidHeader {
    fn default() -> Self {
        AndroidHeader::V0(AndroidHeaderVersion0::default())
    }
}

impl AndroidBootFile {
    fn detect_version(file: &mut File) -> io::Result<u32> {
        file.seek(SeekFrom::Start(0))?;
        let header_v0 = AndroidHeaderVersion0::read_le(file);

        let version = match header_v0 {
            Ok(header) => header.header_version,
            Err(_) => 0,
        };
        file.seek(SeekFrom::Start(0))?;
        Ok(version)
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut file = File::open(path)?;
        let version = Self::detect_version(&mut file)?;

        let result = match version {
            0 => AndroidHeaderVersion0::read_le(&mut file)
                .map(AndroidHeader::V0)
                .map(|header| AndroidBootFile {
                    version: 0,
                    header,
                    file: Some(file),
                }),
            1 => AndroidHeaderVersion1::read_le(&mut file)
                .map(AndroidHeader::V1)
                .map(|header| AndroidBootFile {
                    version: 1,
                    header,
                    file: Some(file),
                }),
            2 => AndroidHeaderVersion2::read_le(&mut file)
                .map(AndroidHeader::V2)
                .map(|header| AndroidBootFile {
                    version: 2,
                    header,
                    file: Some(file),
                }),
            3 => AndroidHeaderVersion3::read_le(&mut file)
                .map(AndroidHeader::V3)
                .map(|header| AndroidBootFile {
                    version: 3,
                    header,
                    file: Some(file),
                }),
            4 => AndroidHeaderVersion4::read_le(&mut file)
                .map(AndroidHeader::V4)
                .map(|header| AndroidBootFile {
                    version: 4,
                    header,
                    file: Some(file),
                }),
            _ => AndroidHeaderVersion0::read_le(&mut file)
                .map(AndroidHeader::V0)
                .map(|header| AndroidBootFile {
                    version: 0,
                    header,
                    file: Some(file),
                }),
        };

        match result {
            Ok(mut boot_file) => {
                // Just to be sure we are at the beginning of the file
                if let Some(ref mut file) = boot_file.file {
                    file.seek(SeekFrom::Start(0))?;
                }
                *self = boot_file;
                Ok(())
            }
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P, page_size: u32) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let result = match &self.header {
            AndroidHeader::V0(header) => header.write_le(&mut file),
            AndroidHeader::V1(header) => header.write_le(&mut file),
            AndroidHeader::V2(header) => header.write_le(&mut file),
            AndroidHeader::V3(header) => header.write_le(&mut file),
            AndroidHeader::V4(header) => header.write_le(&mut file),
        };

        let zeros = vec![0u8; page_size as usize - file.stream_position()? as usize];
        file.write_all(&zeros)?; // Write zeroes

        match result {
            Ok(_) => {
                self.file = Some(file); // Store the file handle
                Ok(())
            }
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }

    pub fn get_file(&self) -> &File {
        self.file.as_ref().unwrap()
    }
}
