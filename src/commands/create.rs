use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::headers::BootHeader;

pub fn create(
    output_boot_img: &PathBuf,
    _kernel_file: &PathBuf,
    _ramdisk_file: &PathBuf,
    _second_file: Option<PathBuf>,
    _config_file: Option<PathBuf>,
    _cmdline_params: &[String],
) -> Result<(), AbootCrafterError> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(output_boot_img)?;

    let header = BootHeader::read_header(&mut file)?;

    match header {
        BootHeader::AndroidHeaderVersion0(ref _header) => unimplemented!(),
        BootHeader::AndroidHeaderVersion1(ref _header) => unimplemented!(),
        BootHeader::AndroidHeaderVersion2(ref _header) => unimplemented!(),
        BootHeader::AndroidHeaderVersion3(ref _header) => unimplemented!(),
        BootHeader::AndroidHeaderVersion4(ref _header) => unimplemented!(),
    }
}
