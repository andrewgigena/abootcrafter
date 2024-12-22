use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::errors::AbootCrafterError;

pub fn update(
    input_boot_file: &PathBuf,
    _config_file: Option<PathBuf>,
    _kernel_file: Option<PathBuf>,
    _ramdisk_file: Option<PathBuf>,
    _second_file: Option<PathBuf>,
    _cmdline_params: &[String],
) -> Result<(), AbootCrafterError> {
    let mut _file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(input_boot_file)?;

    unimplemented!()
}
