use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about = "Manipulate android boot images like a real blacksmith")]
pub struct Cli {
    #[command(subcommand)]
    pub command: MainCommand,
}

#[derive(Subcommand, Debug)]
pub enum MainCommand {
    /// Display information about a boot image
    #[command(alias = "i")]
    Info {
        #[command(subcommand)]
        command: InfoCommand,
    },

    /// Extract components from a boot image
    #[command(alias = "x")]
    Extract {
        #[command(subcommand)]
        command: ExtractCommand,
    },

    /// Update an existing boot image
    #[command(alias = "u")]
    Update {
        #[command(subcommand)]
        command: UpdateCommand,
    },

    /// Create a new boot image
    Create {
        #[command(subcommand)]
        command: CreateCommand,
    },
    // /// Ramdisk manipulation commands
    // Ramdisk {
    //     #[command(subcommand)]
    //     command: RamdiskCommand,
    // },

    // /// Device tree manipulation commands
    // Devicetree {
    //     #[command(subcommand)]
    //     command: DevicetreeCommand,
    // },

    // /// Signature manipulation commands
    // Signature {
    //     #[command(subcommand)]
    //     command: SignatureCommand,
    // },

    // /// Kernel manipulation commands
    // Kernel {
    //     #[command(subcommand)]
    //     command: KernelCommand,
    // },
}

#[derive(Subcommand, Debug)]
pub enum InfoCommand {
    /// Display information about a boot image
    Bootimg {
        /// Boot image file to display information about
        #[arg(short, long, value_parser = file_exists_value_parser)]
        input_boot_file: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum ExtractCommand {
    /// Extract components from a boot image
    Bootimg {
        /// Boot image file to extract components from
        #[arg(short, long, value_parser = file_exists_value_parser)]
        input_boot_file: PathBuf,

        /// Directory to extract components to
        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },
}

#[derive(Subcommand, Debug)]
pub enum UpdateCommand {
    /// Update an existing boot image
    Bootimg {
        /// Boot image file to update
        #[arg(short, long, required = true)]
        input_boot_file: PathBuf,

        /// Kernel file to use for updating
        #[arg(short = 'k', long, value_parser = file_exists_value_parser)]
        kernel_file: Option<PathBuf>,

        /// Ramdisk file to use for updating
        #[arg(short = 'r', long, value_parser = file_exists_value_parser)]
        ramdisk_file: Option<PathBuf>,

        /// Second file to use for updating (v0 to v2 only)
        #[arg(short = 's', long, value_parser = file_exists_value_parser)]
        second_file: Option<PathBuf>,

        /// Recovery DTBO file to use for updating (v1 to v2 only)
        #[arg(long, value_parser = file_exists_value_parser)]
        recovery_dtbo_file: Option<PathBuf>,

        /// DTB file to use for updating (v2 only)
        #[arg(long, value_parser = file_exists_value_parser)]
        dtb_file: Option<PathBuf>,

        /// Kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        cmdline: Option<String>,

        /// Extra kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        extra_cmdline: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum CreateCommand {
    /// Create a new boot image version 0 (< Android 9)
    BootimgV0 {
        /// Output boot image file
        #[arg(short, long, required = true)]
        output_boot_file: PathBuf,

        /// Kernel file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        kernel_file: PathBuf,

        /// Ramdisk file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        ramdisk_file: PathBuf,

        /// Second file to use for creating the boot image
        #[arg(short, long, value_parser = file_exists_value_parser)]
        second_file: Option<PathBuf>,

        /// Page size to use for creating the boot image
        #[arg(long, default_value = "2048")]
        page_size: AndroidBootPageSizes,

        /// Physical load address of the kernel
        #[arg(long, default_value = "0x00008000", value_parser = address32_value_parser)]
        kernel_addr: String,

        /// Physical load address of the ramdisk
        #[arg(long, default_value = "0x01000000", value_parser = address32_value_parser)]
        ramdisk_addr: String,

        /// Physical load address of the second
        #[arg(long, default_value = "0x00000000", value_parser = address32_value_parser)]
        second_addr: String,

        /// Physical load address of the tags
        #[arg(long, default_value = "0x00000100", value_parser = address32_value_parser)]
        tags_addr: String,

        /// Android OS Version of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        os_version: String,

        /// Product name of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        name: String,

        /// Kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        cmdline: String,

        /// timestamp / checksum / sha1 / etc
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        id: String,

        /// Extra kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        extra_cmdline: String,
    },

    /// Create a new boot image version 1 (== Android 9)
    BootimgV1 {
        /// Output boot image file
        #[arg(short, long, required = true)]
        output_boot_file: PathBuf,

        /// Kernel file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        kernel_file: PathBuf,

        /// Ramdisk file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        ramdisk_file: PathBuf,

        /// Second file to use for creating the boot image
        #[arg(short, long, value_parser = file_exists_value_parser)]
        second_file: Option<PathBuf>,

        /// Recovery DTBO file to use for creating the boot image
        #[arg(short = 'D', long, value_parser = file_exists_value_parser)]
        recovery_dtbo_file: Option<PathBuf>,

        /// Page size to use for creating the boot image
        #[arg(long, default_value = "2048")]
        page_size: AndroidBootPageSizes,

        /// Physical load address of the kernel
        #[arg(long, default_value = "0x00008000", value_parser = address32_value_parser)]
        kernel_addr: String,

        /// Physical load address of the ramdisk
        #[arg(long, default_value = "0x01000000", value_parser = address32_value_parser)]
        ramdisk_addr: String,

        /// Physical load address of the second
        #[arg(long, default_value = "0x00000000", value_parser = address32_value_parser)]
        second_addr: String,

        /// Physical load address of the tags
        #[arg(long, default_value = "0x00000100", value_parser = address32_value_parser)]
        tags_addr: String,

        /// Android OS Version of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        os_version: String,

        /// Product name of the boot image        
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        name: String,

        /// Kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        cmdline: String,

        /// timestamp / checksum / sha1 / etc
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        id: String,

        /// Extra kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        extra_cmdline: String,

        /// Offset of the recovery DTBO partition
        #[arg(long, default_value = "0x0000000000000000", value_parser = address64_value_parser)]
        recovery_dtbo_offset: String,
    },

    /// Create a new boot image version 2 (== Android 10)
    BootimgV2 {
        /// Output boot image file
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        output_boot_file: PathBuf,

        /// Kernel file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        kernel_file: PathBuf,

        /// Ramdisk file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        ramdisk_file: PathBuf,

        #[arg(short, long, value_parser = file_exists_value_parser)]
        second_file: Option<PathBuf>,

        #[arg(short = 'D', long, value_parser = file_exists_value_parser)]
        recovery_dtbo_file: Option<PathBuf>,

        /// Device tree file to use for creating the boot image
        #[arg(short = 'd', long, value_parser = file_exists_value_parser)]
        dtb_file: Option<PathBuf>,

        /// Page size to use for creating the boot image
        #[arg(long, default_value = "2048")]
        page_size: AndroidBootPageSizes,

        /// Physical load address of the kernel
        #[arg(long, default_value = "0x00008000", value_parser = address32_value_parser)]
        kernel_addr: String,

        /// Physical load address of the ramdisk
        #[arg(long, default_value = "0x01000000", value_parser = address32_value_parser)]
        ramdisk_addr: String,

        /// Physical load address of the second
        #[arg(long, default_value = "0x00000000", value_parser = address32_value_parser)]
        second_addr: String,

        /// Physical load address of the tags
        #[arg(long, default_value = "0x00000100", value_parser = address32_value_parser)]
        tags_addr: String,

        /// Android OS Version of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        os_version: String,

        /// Product name of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        name: String,

        /// Kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        cmdline: String,

        /// timestamp / checksum / sha1 / etc
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        id: String,

        /// Extra kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        extra_cmdline: String,

        /// Offset of the recovery DTBO partition
        #[arg(long, default_value = "0x0000000000000000", value_parser = address64_value_parser)]
        recovery_dtbo_offset: String,

        /// Physical load address of the device tree
        #[arg(long, default_value = "0x0000000000000000", value_parser = address64_value_parser)]
        dtb_addr: String,
    },

    /// Create a new boot image version 3 (>= Android 11)
    BootimgV3 {
        /// Output boot image file
        #[arg(short, long, required = true)]
        output_boot_file: PathBuf,

        /// Kernel file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        kernel_file: PathBuf,

        /// Ramdisk file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        ramdisk_file: PathBuf,

        /// Android OS Version of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        os_version: String,

        /// Kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        cmdline: String,
    },

    /// Create a new boot image version 4 (>= Android 12)
    BootimgV4 {
        /// Output boot image file
        #[arg(short, long, required = true)]
        output_boot_file: PathBuf,

        /// Kernel file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        kernel_file: PathBuf,

        /// Ramdisk file to use for creating the boot image
        #[arg(short, long, required = true, value_parser = file_exists_value_parser)]
        ramdisk_file: PathBuf,

        /// Android OS Version of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        os_version: String,

        /// Kernel command line of the boot image
        #[arg(long, default_value = "", value_parser = ascii_string_value_parser)]
        cmdline: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum RamdiskCommand {
    /// Display information about a ramdisk
    Info {
        #[arg(short, long)]
        input_file: PathBuf,
    },
    /// Recompress a ramdisk in-place
    Recompress {
        #[arg(short, long)]
        input_file: PathBuf,

        #[arg(short, long)]
        compression: String,
    },
    /// Unpack a ramdisk
    Unpack {
        #[arg(short, long)]
        input_file: PathBuf,

        #[arg(short, long)]
        output_dir: PathBuf,
    },
    /// Repack a ramdisk
    Repack {
        #[arg(short, long)]
        input_dir: PathBuf,

        #[arg(short, long)]
        output_file: PathBuf,

        #[arg(short, long)]
        compression: String,
    },
    /// Add a file to ramdisk
    AddFile {
        #[arg(short, long)]
        ramdisk_file: PathBuf,

        #[arg(short, long)]
        input_file: PathBuf,

        #[arg(short, long)]
        target_path: String,
    },
    /// Remove a file from ramdisk
    RemoveFile {
        #[arg(short, long)]
        ramdisk_file: PathBuf,

        #[arg(short, long)]
        target_path: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum DevicetreeCommand {
    /// Display information about a device tree
    Info {
        #[arg(short, long)]
        input_file: PathBuf,
    },
    /// Remove a node from device tree
    Remove {
        #[arg(short, long)]
        input_file: PathBuf,

        #[arg(short, long)]
        node_path: String,
    },
    /// Add a node to device tree
    Add {
        #[arg(short, long)]
        input_file: PathBuf,

        #[arg(short, long)]
        node_path: String,

        #[arg(short, long)]
        properties: Vec<String>,
    },
    /// Replace a node in device tree
    Replace {
        #[arg(short, long)]
        input_file: PathBuf,

        #[arg(short, long)]
        node_path: String,

        #[arg(short, long)]
        replacement_file: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum SignatureCommand {
    /// Display information about a signature
    Info {
        #[arg(short, long)]
        input_file: PathBuf,
    },
    /// Remove signature from boot image
    Remove {
        #[arg(short, long)]
        input_file: PathBuf,
    },
    /// Replace signature in boot image
    Replace {
        #[arg(short, long)]
        input_file: PathBuf,

        #[arg(short, long)]
        signature_file: PathBuf,
    },
    /// Generate new signature for boot image
    Generate {
        #[arg(short, long)]
        input_file: PathBuf,

        #[arg(short, long)]
        key_file: PathBuf,

        #[arg(short, long)]
        output_file: Option<PathBuf>,
    },
}

#[derive(Subcommand, Debug)]
pub enum KernelCommand {
    /// Display information about a kernel
    Info {
        #[arg(short, long)]
        input_file: PathBuf,
    },
    /// Extract kernel configuration
    ExtractConfig {
        #[arg(short, long)]
        input_file: PathBuf,

        #[arg(short, long)]
        output_file: PathBuf,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AndroidBootPageSizes {
    _2048 = 2048,
    _4096 = 4096,
    _8192 = 8192,
    _16384 = 16384,
}

fn file_exists_value_parser(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("File does not exist: {}", path.display()))
    }
}

fn ascii_string_value_parser(s: &str) -> Result<String, String> {
    if s.is_ascii() {
        Ok(s.to_string())
    } else {
        Err("String must be ASCII compatible".to_string())
    }
}

fn address32_value_parser(s: &str) -> Result<String, String> {
    if s.starts_with("0x") {
        u32::from_str_radix(&s[2..], 16)
            .map(|_| s.to_string())
            .map_err(|e| e.to_string())
    } else {
        Err("Address must start with 0x".to_string())
    }
}

fn address64_value_parser(s: &str) -> Result<String, String> {
    if s.starts_with("0x") {
        u64::from_str_radix(&s[2..], 16)
            .map(|_| s.to_string())
            .map_err(|e| e.to_string())
    } else {
        Err("Address must start with 0x".to_string())
    }
}
