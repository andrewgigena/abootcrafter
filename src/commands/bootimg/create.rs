use std::fs::OpenOptions;
use std::io::{Seek, Write};
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::headers::android::{
    AndroidHeader, AndroidHeaderVersion0, AndroidHeaderVersion1, AndroidHeaderVersion2,
    AndroidHeaderVersion3, AndroidHeaderVersion4,
};
use crate::headers::{android::AndroidBootFile, fields::*};

pub fn create(
    output_boot_img: &PathBuf,
    kernel_file: &PathBuf,
    ramdisk_file: &PathBuf,
    second_file: Option<PathBuf>,
    version: u32,
    page_size: u32,
    kernel_addr: String,
    ramdisk_addr: String,
    second_addr: String,
    tags_addr: String,
    os_version: String,
    name: String,
    cmdline: String,
    id: String,
    extra_cmdline: String,
    recovery_dtbo_size: u32,
    recovery_dtbo_offset: String,
    header_size: u32,
    dtb_size: u32,
    dtb_addr: String,
    signature_size: u32,
) -> Result<(), AbootCrafterError> {
    let mut boot_file = AndroidBootFile::default();
    let kernel_data = std::fs::read(kernel_file)?;
    let ramdisk_data = std::fs::read(ramdisk_file)?;
    let second_data = second_file.map_or_else(Vec::new, |p| std::fs::read(p).unwrap_or_default());

    let header = match version {
        0 => AndroidHeader::V0(AndroidHeaderVersion0 {
            magic: AndroidBootMagic(b"ANDROID!".to_vec()),
            kernel_size: kernel_data.len() as u32,
            kernel_addr: kernel_addr.into(),
            ramdisk_size: ramdisk_data.len() as u32,
            ramdisk_addr: ramdisk_addr.into(),
            second_size: second_data.len() as u32,
            second_addr: second_addr.into(),
            tags_addr: tags_addr.into(),
            page_size,
            header_version: version,
            os_version: os_version.into(),
            name: name.into(),
            cmdline: cmdline.into(),
            id: id.into(),
            extra_cmdline: extra_cmdline.into(),
        }),
        1 => AndroidHeader::V1(AndroidHeaderVersion1 {
            magic: AndroidBootMagic(b"ANDROID!".to_vec()),
            kernel_size: kernel_data.len() as u32,
            kernel_addr: kernel_addr.into(),
            ramdisk_size: ramdisk_data.len() as u32,
            ramdisk_addr: ramdisk_addr.into(),
            second_size: second_data.len() as u32,
            second_addr: second_addr.into(),
            tags_addr: tags_addr.into(),
            page_size,
            header_version: version,
            os_version: os_version.into(),
            name: name.into(),
            cmdline: cmdline.into(),
            id: id.into(),
            extra_cmdline: extra_cmdline.into(),
            recovery_dtbo_size,
            recovery_dtbo_offset: recovery_dtbo_offset.into(),
            header_size,
        }),
        2 => AndroidHeader::V2(AndroidHeaderVersion2 {
            magic: AndroidBootMagic(b"ANDROID!".to_vec()),
            kernel_size: kernel_data.len() as u32,
            kernel_addr: kernel_addr.into(),
            ramdisk_size: ramdisk_data.len() as u32,
            ramdisk_addr: ramdisk_addr.into(),
            second_size: second_data.len() as u32,
            second_addr: second_addr.into(),
            tags_addr: tags_addr.into(),
            page_size,
            header_version: version,
            os_version: os_version.into(),
            name: name.into(),
            cmdline: cmdline.into(),
            id: id.into(),
            extra_cmdline: extra_cmdline.into(),
            recovery_dtbo_size,
            recovery_dtbo_offset: recovery_dtbo_offset.into(),
            header_size,
            dtb_size,
            dtb_addr: dtb_addr.into(),
        }),
        3 => AndroidHeader::V3(AndroidHeaderVersion3 {
            magic: AndroidBootMagic(b"ANDROID!".to_vec()),
            kernel_size: kernel_data.len() as u32,
            ramdisk_size: ramdisk_data.len() as u32,
            os_version: os_version.into(),
            header_size,
            reserved: [0; 4],
            header_version: version,
            cmdline: cmdline.into(),
        }),
        4 => AndroidHeader::V4(AndroidHeaderVersion4 {
            magic: AndroidBootMagic(b"ANDROID!".to_vec()),
            kernel_size: kernel_data.len() as u32,
            ramdisk_size: ramdisk_data.len() as u32,
            os_version: os_version.into(),
            header_size,
            reserved: [0; 4],
            header_version: version,
            cmdline: cmdline.into(),
            signature_size,
        }),
        _ => {
            return Err(AbootCrafterError::ConfigError(
                "Unsupported version".to_string(),
            ))
        }
    };

    boot_file.header = header;
    boot_file.version = version;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_boot_img)?;

    boot_file.save(output_boot_img)?;

    // Get the file and component sizes
    let page_size = boot_file.get_page_size();
    let kernel_size = boot_file.get_kernel_size();
    let ramdisk_size = boot_file.get_ramdisk_size();

    // Write the components
    let header_pages = 1;
    let kernel_offset = page_size * header_pages;
    let kernel_pages = kernel_size.div_ceil(page_size);
    file.seek(std::io::SeekFrom::Start(kernel_offset as u64))?;
    file.write_all(&kernel_data)?;

    let ramdisk_offset = page_size * (header_pages + kernel_pages);
    let ramdisk_pages = ramdisk_size.div_ceil(page_size);
    file.seek(std::io::SeekFrom::Start(ramdisk_offset as u64))?;
    file.write_all(&ramdisk_data)?;

    if !second_data.is_empty() && version < 3 {
        let second_offset = page_size * (header_pages + kernel_pages + ramdisk_pages);
        file.seek(std::io::SeekFrom::Start(second_offset as u64))?;
        file.write_all(&second_data)?;
    } else {
        println!("Second component is empty or version is 3 or higher");
    }

    Ok(())
}
