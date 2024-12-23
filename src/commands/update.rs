use std::io::{Seek, Write};
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::headers::android::AndroidBootFile;
use crate::headers::android::{AndroidHeader, PAGE_SIZE_V3};

const EMPTY_SIZE: u32 = 0;

pub fn update(
    input_boot_file: &PathBuf,
    kernel_file: Option<PathBuf>,
    ramdisk_file: Option<PathBuf>,
    second_file: Option<PathBuf>,
    recovery_dtbo_file: Option<PathBuf>,
    dtb_file: Option<PathBuf>,
    cmdline: Option<String>,
    extra_cmdline: Option<String>,
) -> Result<(), AbootCrafterError> {
    let mut boot_file = AndroidBootFile::default();
    boot_file.load(input_boot_file)?;

    let kernel_data = kernel_file.map_or_else(|| Ok(Vec::new()), |p| std::fs::read(p))?;
    let ramdisk_data = ramdisk_file.map_or_else(|| Ok(Vec::new()), |p| std::fs::read(p))?;
    let second_data = second_file.map_or_else(|| Ok(Vec::new()), |p| std::fs::read(p))?;
    let recovery_dtbo_data =
        recovery_dtbo_file.map_or_else(|| Ok(Vec::new()), |p| std::fs::read(p))?;
    let dtb_data = dtb_file.map_or_else(|| Ok(Vec::new()), |p| std::fs::read(p))?;

    match &mut boot_file.header {
        AndroidHeader::V0(ref mut header) => {
            header.kernel_size = if kernel_data.is_empty() {
                EMPTY_SIZE
            } else {
                kernel_data.len() as u32
            };
            header.ramdisk_size = if ramdisk_data.is_empty() {
                EMPTY_SIZE
            } else {
                ramdisk_data.len() as u32
            };
            header.second_size = if second_data.is_empty() {
                EMPTY_SIZE
            } else {
                second_data.len() as u32
            };
            if !recovery_dtbo_data.is_empty() {
                println!("recovery_dtbo is not supported on v0");
            }
            if !dtb_data.is_empty() {
                println!("dtb is not supported on v0");
            }
            if !cmdline.is_none() {
                header.cmdline = cmdline.unwrap().into();
            }
            if !extra_cmdline.is_none() {
                header.extra_cmdline = extra_cmdline.unwrap().into();
            }
        }
        AndroidHeader::V1(ref mut header) => {
            header.kernel_size = if kernel_data.is_empty() {
                EMPTY_SIZE
            } else {
                kernel_data.len() as u32
            };
            header.ramdisk_size = if ramdisk_data.is_empty() {
                EMPTY_SIZE
            } else {
                ramdisk_data.len() as u32
            };
            header.second_size = if second_data.is_empty() {
                EMPTY_SIZE
            } else {
                second_data.len() as u32
            };
            header.recovery_dtbo_size = if recovery_dtbo_data.is_empty() {
                EMPTY_SIZE
            } else {
                recovery_dtbo_data.len() as u32
            };
            if !dtb_data.is_empty() {
                println!("dtb is not supported on v1");
            }
            if !cmdline.is_none() {
                header.cmdline = cmdline.unwrap().into();
            }
            if !extra_cmdline.is_none() {
                header.extra_cmdline = extra_cmdline.unwrap().into();
            }
        }
        AndroidHeader::V2(ref mut header) => {
            header.kernel_size = if kernel_data.is_empty() {
                EMPTY_SIZE
            } else {
                kernel_data.len() as u32
            };
            header.ramdisk_size = if ramdisk_data.is_empty() {
                EMPTY_SIZE
            } else {
                ramdisk_data.len() as u32
            };
            header.second_size = if second_data.is_empty() {
                EMPTY_SIZE
            } else {
                second_data.len() as u32
            };
            header.recovery_dtbo_size = if recovery_dtbo_data.is_empty() {
                EMPTY_SIZE
            } else {
                recovery_dtbo_data.len() as u32
            };
            header.dtb_size = if dtb_data.is_empty() {
                EMPTY_SIZE
            } else {
                dtb_data.len() as u32
            };
            if !cmdline.is_none() {
                header.cmdline = cmdline.unwrap().into();
            }
            if !extra_cmdline.is_none() {
                header.extra_cmdline = extra_cmdline.unwrap().into();
            }
        }
        AndroidHeader::V3(ref mut header) => {
            header.kernel_size = if kernel_data.is_empty() {
                EMPTY_SIZE
            } else {
                kernel_data.len() as u32
            };
            header.ramdisk_size = if ramdisk_data.is_empty() {
                EMPTY_SIZE
            } else {
                ramdisk_data.len() as u32
            };
            if !second_data.is_empty() {
                println!("second is not supported on v3");
            }
            if !recovery_dtbo_data.is_empty() {
                println!("recovery_dtbo is not supported on v3");
            }
            if !dtb_data.is_empty() {
                println!("dtb is not supported on v3");
            }
            if !cmdline.is_none() {
                header.cmdline = cmdline.unwrap().into();
            }
            if !extra_cmdline.is_none() {
                println!("extra_cmdline is not supported on v3");
            }
        }
        AndroidHeader::V4(ref mut header) => {
            header.kernel_size = if kernel_data.is_empty() {
                EMPTY_SIZE
            } else {
                kernel_data.len() as u32
            };
            header.ramdisk_size = if ramdisk_data.is_empty() {
                EMPTY_SIZE
            } else {
                ramdisk_data.len() as u32
            };
            if !second_data.is_empty() {
                println!("second is not supported on v4");
            }
            if !recovery_dtbo_data.is_empty() {
                println!("recovery_dtbo is not supported on v4");
            }
            if !dtb_data.is_empty() {
                println!("dtb is not supported on v4");
            }
            if !cmdline.is_none() {
                header.cmdline = cmdline.unwrap().into();
            }
            if !extra_cmdline.is_none() {
                println!("extra_cmdline is not supported on v4");
            }
        }
    }

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

    boot_file.save(input_boot_file, page_size)?;

    let mut file = boot_file.get_file();
    let kernel_offset = page_size;
    let ramdisk_offset = kernel_offset + kernel_size;
    let second_offset = ramdisk_offset + ramdisk_size;

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
