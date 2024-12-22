use std::fs::OpenOptions;
use std::io::{Seek, Write};
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::headers::android::AndroidBootFile;
use crate::headers::android::AndroidHeader;

pub fn update(
    input_boot_file: &PathBuf,
    kernel_file: Option<PathBuf>,
    ramdisk_file: Option<PathBuf>,
    second_file: Option<PathBuf>,
) -> Result<(), AbootCrafterError> {
    let mut boot_file = AndroidBootFile::default();
    boot_file.load(input_boot_file)?;

    let kernel_data = kernel_file.map_or_else(|| Ok(Vec::new()), |p| std::fs::read(p))?;
    let ramdisk_data = ramdisk_file.map_or_else(|| Ok(Vec::new()), |p| std::fs::read(p))?;
    let second_data = second_file.map_or_else(|| Ok(Vec::new()), |p| std::fs::read(p))?;

    match &mut boot_file.header {
        AndroidHeader::V0(ref mut header) => {
            if !kernel_data.is_empty() {
                header.kernel_size = kernel_data.len() as u32;
            }
            if !ramdisk_data.is_empty() {
                header.ramdisk_size = ramdisk_data.len() as u32;
            }
            if !second_data.is_empty() {
                header.second_size = second_data.len() as u32;
            }
        }
        AndroidHeader::V1(ref mut header) => {
            if !kernel_data.is_empty() {
                header.kernel_size = kernel_data.len() as u32;
            }
            if !ramdisk_data.is_empty() {
                header.ramdisk_size = ramdisk_data.len() as u32;
            }
            if !second_data.is_empty() {
                header.second_size = second_data.len() as u32;
            }
        }
        AndroidHeader::V2(ref mut header) => {
            if !kernel_data.is_empty() {
                header.kernel_size = kernel_data.len() as u32;
            }
            if !ramdisk_data.is_empty() {
                header.ramdisk_size = ramdisk_data.len() as u32;
            }
            if !second_data.is_empty() {
                header.second_size = second_data.len() as u32;
            }
        }
        AndroidHeader::V3(ref mut header) => {
            if !kernel_data.is_empty() {
                header.kernel_size = kernel_data.len() as u32;
            }
            if !ramdisk_data.is_empty() {
                header.ramdisk_size = ramdisk_data.len() as u32;
            }
        }
        AndroidHeader::V4(ref mut header) => {
            if !kernel_data.is_empty() {
                header.kernel_size = kernel_data.len() as u32;
            }
            if !ramdisk_data.is_empty() {
                header.ramdisk_size = ramdisk_data.len() as u32;
            }
        }
    }

    boot_file.save(input_boot_file)?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(input_boot_file)?;

    let page_size = boot_file.get_page_size();
    let kernel_offset = page_size;
    let ramdisk_offset = kernel_offset + boot_file.get_kernel_size();
    let second_offset = ramdisk_offset + boot_file.get_ramdisk_size();

    if !kernel_data.is_empty() {
        file.seek(std::io::SeekFrom::Start(kernel_offset as u64))?;
        file.write_all(&kernel_data)?;
    }

    if !ramdisk_data.is_empty() {
        file.seek(std::io::SeekFrom::Start(ramdisk_offset as u64))?;
        file.write_all(&ramdisk_data)?;
    }

    if !second_data.is_empty() {
        file.seek(std::io::SeekFrom::Start(second_offset as u64))?;
        file.write_all(&second_data)?;
    }

    Ok(())
}
