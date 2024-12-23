use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::headers::android::{AndroidBootFile, AndroidHeader, PAGE_SIZE_V3};

/// Extracts components from an Android boot image file to a specified output directory.
///
/// # Arguments
///
/// * `input_boot_file` - Path to the input boot image file.
/// * `output_dir` - Optional path to the output directory where components will be extracted.
///
/// # Returns
///
/// * `Result<(), AbootCrafterError>` - Ok if successful, or an error if the extraction fails.
pub fn extract(
    input_boot_file: &PathBuf,
    output_dir: Option<PathBuf>,
) -> Result<(), AbootCrafterError> {
    // Load the Android boot file
    let mut boot_file = AndroidBootFile::default();
    boot_file.load(input_boot_file)?;
    let mut file: &fs::File = boot_file.get_file();

    // Determine the output directory name
    let directory_name = if let Some(output_dir) = output_dir {
        output_dir
    } else {
        let file_name = input_boot_file.file_name().unwrap();
        PathBuf::from(format!("{}_extracted", file_name.to_str().unwrap()))
    };
    fs::create_dir_all(&directory_name)?;

    // Define paths for extracted components
    let kernel_path = directory_name.join("kernel");
    let ramdisk_path = directory_name.join("ramdisk");
    let second_path = directory_name.join("second");

    // Get the file and component sizes
    let page_size = match boot_file.header {
        AndroidHeader::V0(ref header) => header.page_size,
        AndroidHeader::V1(ref header) => header.page_size,
        AndroidHeader::V2(ref header) => header.page_size,
        AndroidHeader::V3(_) => PAGE_SIZE_V3,
        AndroidHeader::V4(_) => PAGE_SIZE_V3,
    };
    let kernel_size = match boot_file.header {
        AndroidHeader::V0(ref header) => header.kernel_size,
        AndroidHeader::V1(ref header) => header.kernel_size,
        AndroidHeader::V2(ref header) => header.kernel_size,
        AndroidHeader::V3(ref header) => header.kernel_size,
        AndroidHeader::V4(ref header) => header.kernel_size,
    };
    let ramdisk_size = match boot_file.header {
        AndroidHeader::V0(ref header) => header.ramdisk_size,
        AndroidHeader::V1(ref header) => header.ramdisk_size,
        AndroidHeader::V2(ref header) => header.ramdisk_size,
        AndroidHeader::V3(ref header) => header.ramdisk_size,
        AndroidHeader::V4(ref header) => header.ramdisk_size,
    };
    let second_size = match boot_file.header {
        AndroidHeader::V0(ref header) => header.second_size,
        AndroidHeader::V1(ref header) => header.second_size,
        AndroidHeader::V2(ref header) => header.second_size,
        AndroidHeader::V3(_) => 0,
        AndroidHeader::V4(_) => 0,
    };

    // Calculate offsets and pages for components
    let header_pages = 1;
    let kernel_offset = page_size * header_pages;
    let kernel_pages = kernel_size.div_ceil(page_size);
    let ramdisk_offset = page_size * (header_pages + kernel_pages);
    let ramdisk_pages = ramdisk_size.div_ceil(page_size);
    let second_offset = page_size * (header_pages + kernel_pages + ramdisk_pages);

    // Extract the kernel component if it exists
    if kernel_size > 0 {
        let mut kernel_buf = vec![0; kernel_size as usize];
        file.seek(SeekFrom::Start(kernel_offset as u64))?;
        file.read_exact(&mut kernel_buf)?;
        fs::write(&kernel_path, kernel_buf)?;
    }

    // Extract the ramdisk component if it exists
    if ramdisk_size > 0 {
        let mut ramdisk_buf = vec![0; ramdisk_size as usize];
        file.seek(SeekFrom::Start(ramdisk_offset as u64))?;
        file.read_exact(&mut ramdisk_buf)?;
        fs::write(&ramdisk_path, ramdisk_buf)?;
    }

    // Extract the second component if it exists
    if second_size > 0 {
        let mut second_buf = vec![0; second_size as usize];
        file.seek(SeekFrom::Start(second_offset as u64))?;
        file.read_exact(&mut second_buf)?;
        fs::write(&second_path, second_buf)?;
    }

    Ok(())
}
