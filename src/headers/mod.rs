#![forbid(unsafe_code)]
pub mod android;
mod fields;

use crate::errors::AbootCrafterError;
pub use android::{
    AndroidBootHeader, AndroidHeaderVersion0, AndroidHeaderVersion1, AndroidHeaderVersion2,
    AndroidHeaderVersion3, AndroidHeaderVersion4, BOOT_MAGIC, BOOT_MAGIC_SIZE,
};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub enum BootHeader {
    AndroidHeaderVersion0(AndroidHeaderVersion0),
    AndroidHeaderVersion1(AndroidHeaderVersion1),
    AndroidHeaderVersion2(AndroidHeaderVersion2),
    AndroidHeaderVersion3(AndroidHeaderVersion3),
    AndroidHeaderVersion4(AndroidHeaderVersion4),
}

impl BootHeader {
    pub fn validate_magic(file: &mut File) -> Result<(), AbootCrafterError> {
        let mut magic = [0u8; BOOT_MAGIC_SIZE];
        file.read_exact(&mut magic)?;

        // Check magic number
        if magic != *BOOT_MAGIC {
            println!(
                "Invalid magic number, expected {}, got {}",
                String::from_utf8_lossy(BOOT_MAGIC),
                String::from_utf8_lossy(&magic)
            );
            return Err(AbootCrafterError::InvalidHeaderMagic);
        }

        Ok(())
    }

    pub fn get_version(file: &mut File) -> Result<u32, AbootCrafterError> {
        file.seek(SeekFrom::Start(0))?;
        let mut header_v0 = AndroidHeaderVersion0::default();
        header_v0.read_header(file)?;

        Ok(header_v0.header_version)
    }

    pub fn read_header(file: &mut File) -> Result<BootHeader, AbootCrafterError> {
        Self::validate_magic(file)?;
        let version = Self::get_version(file)?;

        file.seek(SeekFrom::Start(0))?;

        match version {
            0 => {
                let mut header = AndroidHeaderVersion0::default();
                header.read_header(file)?;
                Ok(BootHeader::AndroidHeaderVersion0(header))
            }
            1 => {
                let mut header = AndroidHeaderVersion1::default();
                header.read_header(file)?;
                Ok(BootHeader::AndroidHeaderVersion1(header))
            }
            2 => {
                let mut header = AndroidHeaderVersion2::default();
                header.read_header(file)?;
                Ok(BootHeader::AndroidHeaderVersion2(header))
            }
            3 => {
                let mut header = AndroidHeaderVersion3::default();
                header.read_header(file)?;
                Ok(BootHeader::AndroidHeaderVersion3(header))
            }
            4 => {
                let mut header = AndroidHeaderVersion4::default();
                header.read_header(file)?;
                Ok(BootHeader::AndroidHeaderVersion4(header))
            }
            _ => {
                // Assume version 0 if we can't determine the version
                let mut header = AndroidHeaderVersion0::default();
                header.read_header(file)?;
                Ok(BootHeader::AndroidHeaderVersion0(header))
            }
        }
    }

    pub fn write_header(self, mut file: File) -> Result<(), AbootCrafterError> {
        Self::validate_magic(&mut file)?;

        match self {
            BootHeader::AndroidHeaderVersion0(mut header) => Ok(header.write_header(&mut file)?),
            BootHeader::AndroidHeaderVersion1(mut header) => Ok(header.write_header(&mut file)?),
            BootHeader::AndroidHeaderVersion2(mut header) => Ok(header.write_header(&mut file)?),
            BootHeader::AndroidHeaderVersion3(mut header) => Ok(header.write_header(&mut file)?),
            BootHeader::AndroidHeaderVersion4(mut header) => Ok(header.write_header(&mut file)?),
        }
    }
}
