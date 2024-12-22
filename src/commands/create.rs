use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::errors::AbootCrafterError;

pub fn create(
    output_boot_img: &PathBuf,
    _kernel_file: &PathBuf,
    _ramdisk_file: &PathBuf,
    _second_file: Option<PathBuf>,
    _config_file: Option<PathBuf>,
    _cmdline_params: &[String],
) -> Result<(), AbootCrafterError> {
    unimplemented!()
}
