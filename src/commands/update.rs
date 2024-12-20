use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::errors::AbootCrafterError;
use crate::headers::BootHeader;

pub fn update(
    input_boot_file: &PathBuf,
    _config_file: Option<PathBuf>,
    _kernel_file: Option<PathBuf>,
    _ramdisk_file: Option<PathBuf>,
    _second_file: Option<PathBuf>,
    _cmdline_params: &[String],
) -> Result<(), AbootCrafterError> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(input_boot_file)?;

    let header = BootHeader::read_header(&mut file)?;

    match header {
        BootHeader::AndroidHeaderVersion0(ref _header) => unimplemented!(),
        BootHeader::AndroidHeaderVersion1(ref _header) => unimplemented!(),
        BootHeader::AndroidHeaderVersion2(ref _header) => unimplemented!(),
        BootHeader::AndroidHeaderVersion3(ref _header) => unimplemented!(),
        BootHeader::AndroidHeaderVersion4(ref _header) => unimplemented!(),
    }
}
