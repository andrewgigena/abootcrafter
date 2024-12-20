use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::headers::android::BOOT_IMAGE_HEADER_V3_PAGESIZE;
use crate::headers::BootHeader;

pub fn extract(
    input_boot_file: &PathBuf,
    output_dir: Option<PathBuf>,
) -> Result<(), AbootCrafterError> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(input_boot_file)?;
    let directory_name = if let Some(output_dir) = output_dir {
        output_dir
    } else {
        let file_name = input_boot_file.file_name().unwrap();
        PathBuf::from(format!("{}_extracted", file_name.to_str().unwrap()))
    };
    fs::create_dir_all(&directory_name)?;

    let header = BootHeader::read_header(&mut file)?;

    match header {
        BootHeader::AndroidHeaderVersion0(ref header) => extract_to_files(
            &mut file,
            &directory_name,
            header.page_size,
            header.kernel_size,
            header.ramdisk_size,
            header.second_size,
        ),
        BootHeader::AndroidHeaderVersion1(ref header) => extract_to_files(
            &mut file,
            &directory_name,
            header.page_size,
            header.kernel_size,
            header.ramdisk_size,
            header.second_size,
        ),
        BootHeader::AndroidHeaderVersion2(ref header) => extract_to_files(
            &mut file,
            &directory_name,
            header.page_size,
            header.kernel_size,
            header.ramdisk_size,
            header.second_size,
        ),
        BootHeader::AndroidHeaderVersion3(ref header) => extract_to_files(
            &mut file,
            &directory_name,
            BOOT_IMAGE_HEADER_V3_PAGESIZE.try_into().unwrap(),
            header.kernel_size,
            header.ramdisk_size,
            0,
        ),
        BootHeader::AndroidHeaderVersion4(ref header) => extract_to_files(
            &mut file,
            &directory_name,
            BOOT_IMAGE_HEADER_V3_PAGESIZE.try_into().unwrap(),
            header.kernel_size,
            header.ramdisk_size,
            0,
        ),
    }
}

fn get_number_of_pages(size: u32, page_size: u32) -> u32 {
    (size + page_size - 1) / page_size
}

fn extract_to_files(
    file: &mut File,
    output_dir: &PathBuf,
    page_size: u32,
    kernel_size: u32,
    ramdisk_size: u32,
    second_size: u32,
) -> Result<(), AbootCrafterError> {
    // Set up paths
    let kernel_path = output_dir.join("kernel.img");
    let ramdisk_path = output_dir.join("ramdisk.img");
    let second_path = output_dir.join("second.img");

    // Calculate offsets
    let header_pages = 1;
    let kernel_offset = page_size * header_pages;
    let kernel_pages = get_number_of_pages(kernel_size, page_size);
    let ramdisk_offset = page_size * (header_pages + kernel_pages);
    let ramdisk_pages = get_number_of_pages(ramdisk_size, page_size);
    let second_offset = page_size * (header_pages + kernel_pages + ramdisk_pages);

    // Extract kernel
    if kernel_size > 0 {
        let mut kernel_buf = vec![0; kernel_size as usize];
        file.seek(SeekFrom::Start(kernel_offset as u64))?;
        file.read_exact(&mut kernel_buf)?;
        fs::write(&kernel_path, kernel_buf)?;
    }

    // Extract ramdisk
    if ramdisk_size > 0 {
        let mut ramdisk_buf = vec![0; ramdisk_size as usize];
        file.seek(SeekFrom::Start(ramdisk_offset as u64))?;
        file.read_exact(&mut ramdisk_buf)?;
        fs::write(&ramdisk_path, ramdisk_buf)?;
    }

    // Extract second stage
    if second_size > 0 {
        let mut second_buf = vec![0; second_size as usize];
        file.seek(SeekFrom::Start(second_offset as u64))?;
        file.read_exact(&mut second_buf)?;
        fs::write(&second_path, second_buf)?;
    }

    Ok(())
}
