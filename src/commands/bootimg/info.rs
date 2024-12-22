use crate::errors::AbootCrafterError;
use crate::headers::android::{AndroidBootFile, AndroidHeader};
use std::fs::File;
use std::path::PathBuf;

/// Displays information about the given Android boot image.
///
/// # Arguments
///
/// * `input_boot_file` - The path to the Android boot image file.
///
/// # Errors
///
/// Returns an error if the file could not be opened or if the file format is
/// invalid.
pub fn info(input_boot_file: &PathBuf) -> Result<(), AbootCrafterError> {
    let mut boot_file = AndroidBootFile::default();
    boot_file.load(input_boot_file)?;

    match boot_file.header {
        AndroidHeader::V0(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!(
                "File Size: {}",
                File::open(input_boot_file)?.metadata()?.len()
            );
            println!("Magic: {}", header.magic);
            println!("Kernel Size: {}", header.kernel_size);
            println!("Kernel Address: {}", header.kernel_addr);
            println!("Ramdisk Size: {}", header.ramdisk_size);
            println!("Ramdisk Address: {}", header.ramdisk_addr);
            println!("Second Size: {}", header.second_size);
            println!("Second Address: {}", header.second_addr);
            println!("Tags Address: {}", header.tags_addr);
            println!("Page Size: {}", header.page_size);
            println!("Header Version: {}", header.header_version);
            println!("OS Version: {}", header.os_version);
            println!("Product Name: {}", header.name);
            println!("Command Line Arguments: {}", header.cmdline);
            println!("ID: {}", header.id);
        }
        AndroidHeader::V1(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!(
                "File Size: {}",
                File::open(input_boot_file)?.metadata()?.len()
            );
            println!("Magic: {}", header.magic);
            println!("Kernel Size: {}", header.kernel_size);
            println!("Kernel Address: {}", header.kernel_addr);
            println!("Ramdisk Size: {}", header.ramdisk_size);
            println!("Ramdisk Address: {}", header.ramdisk_addr);
            println!("Second Size: {}", header.second_size);
            println!("Second Address: {}", header.second_addr);
            println!("Tags Address: {}", header.tags_addr);
            println!("Page Size: {}", header.page_size);
            println!("Header Version: {}", header.header_version);
            println!("OS Version: {}", header.os_version);
            println!("Product Name: {}", header.name);
            println!("Command Line Arguments: {}", header.cmdline);
            println!("ID: {}", header.id);
            println!("Extra Command Line Arguments: {}", header.extra_cmdline);
            println!("Recovery DTBO Size: {}", header.recovery_dtbo_size);
            println!("Recovery DTBO Offset: {}", header.recovery_dtbo_offset);
            println!("Header Size: {}", header.header_size);
        }
        AndroidHeader::V2(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!(
                "File Size: {}",
                File::open(input_boot_file)?.metadata()?.len()
            );
            println!("Magic: {}", header.magic);
            println!("Kernel Size: {}", header.kernel_size);
            println!("Kernel Address: {}", header.kernel_addr);
            println!("Ramdisk Size: {}", header.ramdisk_size);
            println!("Ramdisk Address: {}", header.ramdisk_addr);
            println!("Second Size: {}", header.second_size);
            println!("Second Address: {}", header.second_addr);
            println!("Tags Address: {}", header.tags_addr);
            println!("Page Size: {}", header.page_size);
            println!("Header Version: {}", header.header_version);
            println!("OS Version: {}", header.os_version);
            println!("Product Name: {}", header.name);
            println!("Command Line Arguments: {}", header.cmdline);
            println!("ID: {}", header.id);
            println!("Extra Command Line Arguments: {}", header.extra_cmdline);
            println!("Recovery DTBO Size: {}", header.recovery_dtbo_size);
            println!("Recovery DTBO Offset: {}", header.recovery_dtbo_offset);
            println!("Header Size: {}", header.header_size);
            println!("DTB Size: {}", header.dtb_size);
            println!("DTB Address: {}", header.dtb_addr);
        }
        AndroidHeader::V3(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!(
                "File Size: {}",
                File::open(input_boot_file)?.metadata()?.len()
            );
            println!("Magic: {}", header.magic);
            println!("Kernel Size: {}", header.kernel_size);
            println!("Ramdisk Size: {}", header.ramdisk_size);
            println!("OS Version: {}", header.os_version);
            println!("Header Size: {}", header.header_size);
            println!("Header Version: {}", header.header_version);
            println!("Command Line Arguments: {}", header.cmdline);
        }
        AndroidHeader::V4(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!(
                "File Size: {}",
                File::open(input_boot_file)?.metadata()?.len()
            );
            println!("Magic: {}", header.magic);
            println!("Kernel Size: {}", header.kernel_size);
            println!("Ramdisk Size: {}", header.ramdisk_size);
            println!("OS Version: {}", header.os_version);
            println!("Header Size: {}", header.header_size);
            println!("Header Version: {}", header.header_version);
            println!("Command Line Arguments: {}", header.cmdline);
            println!("Signature Size: {}", header.signature_size);
        }
    }

    Ok(())
}
