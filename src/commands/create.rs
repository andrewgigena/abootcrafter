use crate::errors::AbootCrafterError;
use crate::headers::android::{
    AndroidBootFile, AndroidHeader, AndroidHeaderVersion0, AndroidHeaderVersion1,
    AndroidHeaderVersion2, AndroidHeaderVersion3, AndroidHeaderVersion4,
};
use crate::headers::fields::{
    AddressU32, AddressU64, AndroidBootMagic, Cmdline, CmdlineExtended, ExtraCmdline, Id, Name,
    OSVersion,
};
use std::io::Write;
use std::path::PathBuf;

fn pad_data_to_page_size(data: &[u8], page_size: u32) -> Vec<u8> {
    let padded_size = (data.len() as u32).div_ceil(page_size) * page_size;
    let mut padded_data = vec![0u8; padded_size as usize];
    padded_data[..data.len()].copy_from_slice(data);
    padded_data
}

pub fn create_v0(
    output_boot_file: PathBuf,
    kernel_file: PathBuf,
    ramdisk_file: PathBuf,
    second_file: Option<PathBuf>,
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
) -> Result<(), AbootCrafterError> {
    let mut boot_file = AndroidBootFile::default();
    boot_file.version = 0;

    let header = AndroidHeaderVersion0 {
        magic: AndroidBootMagic::default(),
        kernel_size: 0,
        kernel_addr: AddressU32::from(kernel_addr),
        ramdisk_size: 0,
        ramdisk_addr: AddressU32::from(ramdisk_addr),
        second_size: 0,
        second_addr: AddressU32::from(second_addr),
        tags_addr: AddressU32::from(tags_addr),
        page_size,
        header_version: 0,
        os_version: OSVersion::from(os_version),
        name: Name::from(name),
        cmdline: Cmdline::from(cmdline),
        id: Id::from(id),
        extra_cmdline: ExtraCmdline::from(extra_cmdline),
    };

    boot_file.header = AndroidHeader::V0(header);

    // Update header with actual sizes (not padded)
    let header = match boot_file.header {
        AndroidHeader::V0(ref mut header) => header,
        _ => {
            return Err(AbootCrafterError::ConfigError(
                "Invalid header version".to_string(),
            ))
        }
    };

    // Load kernel, ramdisk, and second files
    let kernel_data = std::fs::read(kernel_file)?;
    let ramdisk_data = std::fs::read(ramdisk_file)?;
    let second_data = if let Some(second_file) = second_file {
        std::fs::read(second_file)?
    } else {
        Vec::new()
    };

    // Write raw size to header
    header.kernel_size = kernel_data.len() as u32;
    header.ramdisk_size = ramdisk_data.len() as u32;
    header.second_size = second_data.len() as u32;

    // Write header to output file
    boot_file.save(output_boot_file, page_size)?;
    let mut file = boot_file.get_file();

    // Write padded data to output file
    let kernel_padded = pad_data_to_page_size(&kernel_data, page_size);
    let ramdisk_padded = pad_data_to_page_size(&ramdisk_data, page_size);
    let second_padded = pad_data_to_page_size(&second_data, page_size);
    file.write_all(&kernel_padded)?;
    file.write_all(&ramdisk_padded)?;
    file.write_all(&second_padded)?;

    Ok(())
}

pub fn create_v1(
    output_boot_file: PathBuf,
    kernel_file: PathBuf,
    ramdisk_file: PathBuf,
    second_file: Option<PathBuf>,
    recovery_dtbo_file: Option<PathBuf>,
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
    recovery_dtbo_offset: String,
) -> Result<(), AbootCrafterError> {
    let mut boot_file = AndroidBootFile::default();
    boot_file.version = 1;

    let header = AndroidHeaderVersion1 {
        magic: AndroidBootMagic::default(),
        kernel_size: 0,
        kernel_addr: AddressU32::from(kernel_addr),
        ramdisk_size: 0,
        ramdisk_addr: AddressU32::from(ramdisk_addr),
        second_size: 0,
        second_addr: AddressU32::from(second_addr),
        tags_addr: AddressU32::from(tags_addr),
        page_size,
        header_version: 1,
        os_version: OSVersion::from(os_version),
        name: Name::from(name),
        cmdline: Cmdline::from(cmdline),
        id: Id::from(id),
        extra_cmdline: ExtraCmdline::from(extra_cmdline),
        recovery_dtbo_size: 0,
        recovery_dtbo_offset: AddressU64::from(recovery_dtbo_offset),
        header_size: 0,
    };

    boot_file.header = AndroidHeader::V1(header);

    // Load kernel, ramdisk, second, and recovery_dtbo files
    let kernel_data = std::fs::read(kernel_file)?;
    let ramdisk_data = std::fs::read(ramdisk_file)?;
    let second_data = if let Some(second_file) = second_file {
        std::fs::read(second_file)?
    } else {
        Vec::new()
    };

    let recovery_dtbo_data = if let Some(recovery_dtbo_file) = recovery_dtbo_file {
        std::fs::read(recovery_dtbo_file)?
    } else {
        Vec::new()
    };

    // Update header sizes
    let header = match boot_file.header {
        AndroidHeader::V1(ref mut header) => header,
        _ => {
            return Err(AbootCrafterError::ConfigError(
                "Invalid header version".to_string(),
            ))
        }
    };

    header.kernel_size = kernel_data.len() as u32;
    header.ramdisk_size = ramdisk_data.len() as u32;
    header.second_size = second_data.len() as u32;
    header.recovery_dtbo_size = recovery_dtbo_data.len() as u32;

    // Write to output file
    boot_file.save(output_boot_file, page_size)?;
    let mut file = boot_file.get_file();

    file.write_all(&kernel_data)?;
    file.write_all(&ramdisk_data)?;
    file.write_all(&second_data)?;
    file.write_all(&recovery_dtbo_data)?;

    Ok(())
}

pub fn create_v2(
    output_boot_file: PathBuf,
    kernel_file: PathBuf,
    ramdisk_file: PathBuf,
    second_file: Option<PathBuf>,
    recovery_dtbo_file: Option<PathBuf>,
    dtb_file: Option<PathBuf>,
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
    recovery_dtbo_offset: String,
    dtb_addr: String,
) -> Result<(), AbootCrafterError> {
    let mut boot_file = AndroidBootFile::default();
    boot_file.version = 2;

    let header = AndroidHeaderVersion2 {
        magic: AndroidBootMagic::default(),
        kernel_size: 0,
        kernel_addr: AddressU32::from(kernel_addr),
        ramdisk_size: 0,
        ramdisk_addr: AddressU32::from(ramdisk_addr),
        second_size: 0,
        second_addr: AddressU32::from(second_addr),
        tags_addr: AddressU32::from(tags_addr),
        page_size,
        header_version: 2,
        os_version: OSVersion::from(os_version),
        name: Name::from(name),
        cmdline: Cmdline::from(cmdline),
        id: Id::from(id),
        extra_cmdline: ExtraCmdline::from(extra_cmdline),
        recovery_dtbo_size: 0,
        recovery_dtbo_offset: AddressU64::from(recovery_dtbo_offset),
        header_size: 0,
        dtb_size: 0,
        dtb_addr: AddressU64::from(dtb_addr),
    };

    boot_file.header = AndroidHeader::V2(header);

    // Load kernel, ramdisk, second, recovery_dtbo, and dtb files
    let kernel_data = std::fs::read(kernel_file)?;
    let ramdisk_data = std::fs::read(ramdisk_file)?;
    let second_data = if let Some(second_file) = second_file {
        std::fs::read(second_file)?
    } else {
        Vec::new()
    };
    let recovery_dtbo_data = if let Some(recovery_dtbo_file) = recovery_dtbo_file {
        std::fs::read(recovery_dtbo_file)?
    } else {
        Vec::new()
    };
    let dtb_data = if let Some(dtb_file) = dtb_file {
        std::fs::read(dtb_file)?
    } else {
        Vec::new()
    };

    // Update header sizes
    let header = match boot_file.header {
        AndroidHeader::V2(ref mut header) => header,
        _ => {
            return Err(AbootCrafterError::ConfigError(
                "Invalid header version".to_string(),
            ))
        }
    };

    header.kernel_size = kernel_data.len() as u32;
    header.ramdisk_size = ramdisk_data.len() as u32;
    header.second_size = second_data.len() as u32;
    header.recovery_dtbo_size = recovery_dtbo_data.len() as u32;
    header.dtb_size = dtb_data.len() as u32;

    // Write to output file
    boot_file.save(output_boot_file, page_size)?;
    let mut file = boot_file.get_file();

    file.write_all(&kernel_data)?;
    file.write_all(&ramdisk_data)?;
    file.write_all(&second_data)?;
    file.write_all(&recovery_dtbo_data)?;
    file.write_all(&dtb_data)?;

    Ok(())
}

pub fn create_v3(
    output_boot_file: PathBuf,
    kernel_file: PathBuf,
    ramdisk_file: PathBuf,
    os_version: String,
    cmdline: String,
) -> Result<(), AbootCrafterError> {
    let mut boot_file = AndroidBootFile::default();
    boot_file.version = 3;

    let header = AndroidHeaderVersion3 {
        magic: AndroidBootMagic::default(),
        kernel_size: 0,
        ramdisk_size: 0,
        os_version: OSVersion::from(os_version),
        header_size: 0,
        reserved: [0; 4],
        header_version: 3,
        cmdline: CmdlineExtended::from(cmdline),
    };

    boot_file.header = AndroidHeader::V3(header);

    // Load kernel and ramdisk files
    let kernel_data = std::fs::read(kernel_file)?;
    let ramdisk_data = std::fs::read(ramdisk_file)?;

    // Update header sizes
    let header = match boot_file.header {
        AndroidHeader::V3(ref mut header) => header,
        _ => {
            return Err(AbootCrafterError::ConfigError(
                "Invalid header version".to_string(),
            ))
        }
    };

    header.kernel_size = kernel_data.len() as u32;
    header.ramdisk_size = ramdisk_data.len() as u32;

    // Write to output file
    boot_file.save(output_boot_file, 4096)?;
    let mut file = boot_file.get_file();

    file.write_all(&kernel_data)?;
    file.write_all(&ramdisk_data)?;

    Ok(())
}

pub fn create_v4(
    output_boot_file: PathBuf,
    kernel_file: PathBuf,
    ramdisk_file: PathBuf,
    os_version: String,
    cmdline: String,
) -> Result<(), AbootCrafterError> {
    let mut boot_file = AndroidBootFile::default();
    boot_file.version = 4;

    let header = AndroidHeaderVersion4 {
        magic: AndroidBootMagic::default(),
        kernel_size: 0,
        ramdisk_size: 0,
        os_version: OSVersion::from(os_version),
        header_size: 0,
        reserved: [0; 4],
        header_version: 4,
        cmdline: CmdlineExtended::from(cmdline),
        signature_size: 0,
    };

    boot_file.header = AndroidHeader::V4(header);

    // Load kernel and ramdisk files
    let kernel_data = std::fs::read(kernel_file)?;
    let ramdisk_data = std::fs::read(ramdisk_file)?;

    // Update header sizes
    let header = match boot_file.header {
        AndroidHeader::V4(ref mut header) => header,
        _ => {
            return Err(AbootCrafterError::ConfigError(
                "Invalid header version".to_string(),
            ))
        }
    };

    header.kernel_size = kernel_data.len() as u32;
    header.ramdisk_size = ramdisk_data.len() as u32;

    // Write to output file
    boot_file.save(output_boot_file, 4096)?;
    let mut file = boot_file.get_file();

    file.write_all(&kernel_data)?;
    file.write_all(&ramdisk_data)?;

    Ok(())
}
