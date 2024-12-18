use serde::{Deserialize, Serialize};

pub const BOOT_MAGIC: &[u8; 8] = b"ANDROID!";
pub const BOOT_NAME_SIZE: usize = 16;
pub const BOOT_ARGS_SIZE: usize = 512;

#[derive(Debug, Clone)]
pub struct BootImgHeader {
    pub magic: [u8; 8],
    pub kernel_size: u32,
    pub kernel_addr: u32,
    pub ramdisk_size: u32,
    pub ramdisk_addr: u32,
    pub second_size: u32,
    pub second_addr: u32,
    pub tags_addr: u32,
    pub page_size: u32,
    pub name: [u8; BOOT_NAME_SIZE],
    pub cmdline: [u8; BOOT_ARGS_SIZE],
    pub id: [u32; 8],
}

impl Default for BootImgHeader {
    fn default() -> Self {
        Self {
            magic: *BOOT_MAGIC,
            kernel_size: 0,
            kernel_addr: 0,
            ramdisk_size: 0,
            ramdisk_addr: 0,
            second_size: 0,
            second_addr: 0,
            tags_addr: 0,
            page_size: 2048,
            name: [0; BOOT_NAME_SIZE],
            cmdline: [0; BOOT_ARGS_SIZE],
            id: [0; 8],
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct BootConfig {
    pub boot_size: Option<u64>,
    pub page_size: Option<u32>,
    pub kernel_addr: Option<u32>,
    pub ramdisk_addr: Option<u32>,
    pub second_addr: Option<u32>,
    pub tags_addr: Option<u32>,
    pub cmdline: Option<String>,
}
