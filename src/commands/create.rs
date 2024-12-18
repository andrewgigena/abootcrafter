use std::fs::{self, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::types::{BootConfig, BootImgHeader, BOOT_ARGS_SIZE};
use crate::utils::write_header;

pub fn create(
    output_boot_img: &PathBuf,
    kernel_file: &PathBuf,
    ramdisk_file: &PathBuf,
    second_file: Option<PathBuf>,
    config_file: Option<PathBuf>,
    cmdline_params: &[String],
) -> Result<(), AbootCrafterError> {
    let mut header = BootImgHeader::default();

    // Read kernel
    let kernel_data = fs::read(kernel_file)?;
    header.kernel_size = kernel_data
        .len()
        .try_into()
        .map_err(|_| AbootCrafterError::InvalidImage("Kernel too large".to_string()))?;

    // Read ramdisk
    let ramdisk_data = fs::read(ramdisk_file)?;
    header.ramdisk_size = ramdisk_data
        .len()
        .try_into()
        .map_err(|_| AbootCrafterError::InvalidImage("Ramdisk too large".to_string()))?;

    // Read second stage if provided
    let second_data = second_file.map(fs::read).transpose()?;
    if let Some(ref data) = second_data {
        header.second_size = data
            .len()
            .try_into()
            .map_err(|_| AbootCrafterError::InvalidImage("Second stage too large".to_string()))?;
    }

    // Process configuration if provided
    if let Some(cfg_path) = config_file {
        let cfg_content = fs::read_to_string(&cfg_path)?;
        let config: BootConfig = toml::from_str(&cfg_content)
            .map_err(|e| AbootCrafterError::ConfigError(e.to_string()))?;

        if let Some(kernel_addr) = config.kernel_addr {
            header.kernel_addr = kernel_addr;
        }
        if let Some(ramdisk_addr) = config.ramdisk_addr {
            header.ramdisk_addr = ramdisk_addr;
        }
        if let Some(second_addr) = config.second_addr {
            header.second_addr = second_addr;
        }
        if let Some(tags_addr) = config.tags_addr {
            header.tags_addr = tags_addr;
        }
        if let Some(page_size) = config.page_size {
            header.page_size = page_size;
        }
        if let Some(cmdline) = config.cmdline {
            let bytes = cmdline.as_bytes();
            header.cmdline[..bytes.len().min(BOOT_ARGS_SIZE)]
                .copy_from_slice(&bytes[..bytes.len().min(BOOT_ARGS_SIZE)]);
        }
    }

    // Process command line parameters
    for param in cmdline_params {
        if let Some((key, value)) = param.split_once('=') {
            if key.trim() == "cmdline" {
                let trimmed = value.trim();
                let bytes = trimmed.as_bytes();
                header.cmdline[..bytes.len().min(BOOT_ARGS_SIZE)]
                    .copy_from_slice(&bytes[..bytes.len().min(BOOT_ARGS_SIZE)]);
            }
        }
    }

    // Create boot image file
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_boot_img)?;

    // Write header
    write_header(&mut file, &header)?;

    // Write kernel
    file.seek(SeekFrom::Start(header.page_size as u64))?;
    file.write_all(&kernel_data)?;

    // Pad kernel
    let kernel_pages = header.kernel_size.div_ceil(header.page_size);
    let kernel_padding = kernel_pages * header.page_size - header.kernel_size;
    file.write_all(&vec![0; kernel_padding as usize])?;

    // Write ramdisk
    let ramdisk_offset = (1 + kernel_pages) * header.page_size;
    file.seek(SeekFrom::Start(ramdisk_offset as u64))?;
    file.write_all(&ramdisk_data)?;

    // Pad ramdisk
    let ramdisk_pages = header.ramdisk_size.div_ceil(header.page_size);
    let ramdisk_padding = ramdisk_pages * header.page_size - header.ramdisk_size;
    file.write_all(&vec![0; ramdisk_padding as usize])?;

    // Write second stage if provided
    if let Some(second_data) = second_data {
        let second_offset = (1 + kernel_pages + ramdisk_pages) * header.page_size;
        file.seek(SeekFrom::Start(second_offset as u64))?;
        file.write_all(&second_data)?;

        // Pad second stage
        let second_pages = header.second_size.div_ceil(header.page_size);
        let second_padding = second_pages * header.page_size - header.second_size;
        file.write_all(&vec![0; second_padding as usize])?;
    }

    Ok(())
}
