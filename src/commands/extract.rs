use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::types::BootConfig;
use crate::utils::read_header;

pub fn extract(
    input_boot_file: &PathBuf,
    output_dir: Option<PathBuf>,
) -> Result<(), AbootCrafterError> {
    let mut file = File::open(input_boot_file)?;
    let header = read_header(&mut file)?;

    // Determine the output directory
    let output_dir = output_dir.unwrap_or_else(|| PathBuf::from("out"));
    fs::create_dir_all(&output_dir)?; // Ensure the directory exists

    // Construct file paths
    let config_path = output_dir.join("bootimg.toml");
    let kernel_path = output_dir.join("kernel.img");
    let ramdisk_path = output_dir.join("ramdisk.img");
    let second_path = output_dir.join("second.img");

    // Prepare config
    let config = BootConfig {
        boot_size: Some(file.metadata()?.len()),
        page_size: Some(header.page_size),
        kernel_addr: Some(header.kernel_addr),
        ramdisk_addr: Some(header.ramdisk_addr),
        second_addr: Some(header.second_addr),
        tags_addr: Some(header.tags_addr),
        cmdline: Some(
            String::from_utf8_lossy(&header.cmdline)
                .trim_matches('\0')
                .to_string(),
        ),
    };

    // Write config
    let toml_str = toml::to_string_pretty(&config)
        .map_err(|e| AbootCrafterError::ConfigError(e.to_string()))?;
    fs::write(&config_path, toml_str)?;

    // Extract kernel
    if header.kernel_size > 0 {
        let mut kernel_buf = vec![0; header.kernel_size as usize];
        file.seek(SeekFrom::Start(header.page_size as u64))?;
        file.read_exact(&mut kernel_buf)?;
        fs::write(&kernel_path, kernel_buf)?;
    }

    // Extract ramdisk
    if header.ramdisk_size > 0 {
        let mut ramdisk_buf = vec![0; header.ramdisk_size as usize];
        let ramdisk_offset = (1 + header.kernel_size.div_ceil(header.page_size)) * header.page_size;
        file.seek(SeekFrom::Start(ramdisk_offset as u64))?;
        file.read_exact(&mut ramdisk_buf)?;
        fs::write(&ramdisk_path, ramdisk_buf)?;
    }

    // Extract second stage
    if header.second_size > 0 {
        let mut second_buf = vec![0; header.second_size as usize];
        let second_offset = (1
            + header.kernel_size.div_ceil(header.page_size)
            + header.ramdisk_size.div_ceil(header.page_size))
            * header.page_size;
        file.seek(SeekFrom::Start(second_offset as u64))?;
        file.read_exact(&mut second_buf)?;
        fs::write(&second_path, second_buf)?;
    }

    Ok(())
}
