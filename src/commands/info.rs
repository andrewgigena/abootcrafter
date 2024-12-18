use std::fs::File;
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::types::BootConfig;
use crate::utils::read_header;

pub fn info(input_boot_file: &PathBuf) -> Result<(), AbootCrafterError> {
    let mut file = File::open(input_boot_file)?;
    let header = read_header(&mut file)?;

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

    // Use toml to pretty print
    let toml_str = toml::to_string_pretty(&config)
        .map_err(|e| AbootCrafterError::ConfigError(e.to_string()))?;

    println!("Android Boot Image Info:");
    println!("File: {}", input_boot_file.display());
    println!("Kernel Size: {} bytes", header.kernel_size);
    println!("Ramdisk Size: {} bytes", header.ramdisk_size);
    println!("Second Stage Size: {} bytes", header.second_size);
    println!("\nConfiguration:");
    print!("{}", toml_str);

    Ok(())
}
