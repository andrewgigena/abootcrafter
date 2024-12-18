use std::fs::{self, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::types::{BootConfig, BOOT_ARGS_SIZE};
use crate::utils::{read_header, write_header};

pub fn update(
    input_boot_file: &PathBuf,
    config_file: Option<PathBuf>,
    kernel_file: Option<PathBuf>,
    ramdisk_file: Option<PathBuf>,
    second_file: Option<PathBuf>,
    cmdline_params: &[String],
) -> Result<(), AbootCrafterError> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(input_boot_file)?;
    let mut header = read_header(&mut file)?;

    // Update from config file
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

    // Update from command line parameters
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

    // Update kernel
    if let Some(kernel_path) = kernel_file {
        let kernel_data = fs::read(&kernel_path)?;
        header.kernel_size = kernel_data
            .len()
            .try_into()
            .map_err(|_| AbootCrafterError::InvalidImage("Kernel too large".to_string()))?;
        file.seek(SeekFrom::Start(header.page_size as u64))?;
        file.write_all(&kernel_data)?;
    }

    // Update ramdisk
    if let Some(ramdisk_path) = ramdisk_file {
        let ramdisk_data = fs::read(&ramdisk_path)?;
        header.ramdisk_size = ramdisk_data
            .len()
            .try_into()
            .map_err(|_| AbootCrafterError::InvalidImage("Ramdisk too large".to_string()))?;
        let ramdisk_offset = (1 + header.kernel_size.div_ceil(header.page_size)) * header.page_size;
        file.seek(SeekFrom::Start(ramdisk_offset as u64))?;
        file.write_all(&ramdisk_data)?;
    }

    // Update second stage
    if let Some(second_path) = second_file {
        let second_data = fs::read(&second_path)?;
        header.second_size = second_data
            .len()
            .try_into()
            .map_err(|_| AbootCrafterError::InvalidImage("Second stage too large".to_string()))?;
        let second_offset = (1
            + header.kernel_size.div_ceil(header.page_size)
            + header.ramdisk_size.div_ceil(header.page_size))
            * header.page_size;
        file.seek(SeekFrom::Start(second_offset as u64))?;
        file.write_all(&second_data)?;
    }

    // Write the updated header
    write_header(&mut file, &header)?;

    Ok(())
}
