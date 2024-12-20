use crate::errors::AbootCrafterError;
use crate::headers::BootHeader;
use std::fs::File;
use std::path::PathBuf;

pub fn info(input_boot_file: &PathBuf) -> Result<(), AbootCrafterError> {
    let mut file: File = File::open(input_boot_file)?;
    let header = BootHeader::read_header(&mut file)?;
    match header {
        BootHeader::AndroidHeaderVersion0(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!("File Size: {}", file.metadata()?.len());
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
        BootHeader::AndroidHeaderVersion1(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!("File Size: {}", file.metadata()?.len());
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
        BootHeader::AndroidHeaderVersion2(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!("File Size: {}", file.metadata()?.len());
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
        BootHeader::AndroidHeaderVersion3(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!("File Size: {}", file.metadata()?.len());
            println!("Magic: {}", header.magic);
            println!("Kernel Size: {}", header.kernel_size);
            println!("Ramdisk Size: {}", header.ramdisk_size);
            println!("OS Version: {}", header.os_version);
        }
        BootHeader::AndroidHeaderVersion4(ref header) => {
            println!("[Header]");
            println!("File: {}", input_boot_file.display());
            println!("File Size: {}", file.metadata()?.len());
            println!("Magic: {}", header.magic);
            println!("Kernel Size: {}", header.kernel_size);
            println!("Ramdisk Size: {}", header.ramdisk_size);
            println!("OS Version: {}", header.os_version);
        }
    }

    Ok(())
}
