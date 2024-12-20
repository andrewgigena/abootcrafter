use std::fs::File;

use super::fields::{AddressU32, AddressU64, Cmdline, Id, Magic, Name, OSVersion};
use binrw::{BinRead, BinWrite};

pub const BOOT_MAGIC: &[u8; 8] = b"ANDROID!";
pub const BOOT_MAGIC_SIZE: usize = 8;
pub const BOOT_IMAGE_HEADER_V3_PAGESIZE: usize = 4096;

pub trait AndroidBootHeader: BinRead + BinWrite {
    fn read_header(&mut self, file: &mut File) -> std::io::Result<()>;
    fn write_header(&mut self, file: &mut File) -> std::io::Result<()>;
}

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion0 {
    pub magic: Magic,
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
}

impl AndroidBootHeader for AndroidHeaderVersion0 {
    fn read_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match Self::read_le(file) {
            Ok(val) => {
                *self = val;
                Ok(())
            }
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }

    fn write_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self.write_le(file) {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion1 {
    pub magic: Magic,
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
    pub extra_cmdline: Cmdline,
    pub recovery_dtbo_size: u32,
    pub recovery_dtbo_offset: u64,
    pub header_size: u32,
}

impl AndroidBootHeader for AndroidHeaderVersion1 {
    fn read_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match Self::read_le(file) {
            Ok(val) => {
                *self = val;
                Ok(())
            }
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }

    fn write_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self.write_le(file) {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion2 {
    pub magic: Magic,
    pub kernel_size: u32,
    pub kernel_addr: AddressU32,
    pub ramdisk_size: u32,
    pub ramdisk_addr: AddressU32,
    pub second_size: u32,
    pub second_addr: AddressU32,
    pub tags_addr: u32,
    pub page_size: u32,
    pub header_version: u32,
    pub os_version: OSVersion,
    pub name: Name,
    pub cmdline: Cmdline,
    pub id: Id,
    pub extra_cmdline: Cmdline,
    pub recovery_dtbo_size: u32,
    pub recovery_dtbo_offset: u64,
    pub header_size: u32,
    pub dtb_size: u32,
    pub dtb_addr: AddressU64,
}

impl AndroidBootHeader for AndroidHeaderVersion2 {
    fn read_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match Self::read_le(file) {
            Ok(val) => {
                *self = val;
                Ok(())
            }
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }

    fn write_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self.write_le(file) {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion3 {
    pub magic: Magic,
    pub kernel_size: u32,
    pub ramdisk_size: u32,
    pub os_version: OSVersion,
    pub header_size: u32,
    pub reserved: [u32; 4],
    pub header_version: u32,
    pub cmdline: Cmdline,
}

impl AndroidBootHeader for AndroidHeaderVersion3 {
    fn read_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match Self::read_le(file) {
            Ok(val) => {
                *self = val;
                Ok(())
            }
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }

    fn write_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self.write_le(file) {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
#[br(little)]
pub struct AndroidHeaderVersion4 {
    pub magic: Magic,
    pub kernel_size: u32,
    pub ramdisk_size: u32,
    pub os_version: OSVersion,
    pub header_size: u32,
    pub reserved: [u32; 4],
    pub header_version: u32,
    pub cmdline: Cmdline,
    pub signature_size: u32,
}

impl AndroidBootHeader for AndroidHeaderVersion4 {
    fn read_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match Self::read_le(file) {
            Ok(val) => {
                *self = val;
                Ok(())
            }
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }

    fn write_header(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self.write_le(file) {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }
}
