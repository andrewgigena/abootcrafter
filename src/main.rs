#![forbid(unsafe_code)]
mod commands;
mod errors;
mod headers;

use clap::{Parser, Subcommand};
use errors::AbootCrafterError;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about = "Manipulate android boot images like a real blacksmith")]
pub struct Cli {
    #[command(subcommand)]
    pub command: MainCommand,
}

#[derive(Subcommand, Debug)]
pub enum MainCommand {
    /// Boot image manipulation commands
    Bootimg {
        #[command(subcommand)]
        command: BootimgCommand,
    },
    /// Ramdisk manipulation commands
    Ramdisk {
        #[command(subcommand)]
        command: RamdiskCommand,
    },
    /// Device tree manipulation commands
    Devicetree {
        #[command(subcommand)]
        command: DevicetreeCommand,
    },
    /// Signature manipulation commands
    Signature {
        #[command(subcommand)]
        command: SignatureCommand,
    },
    /// Kernel manipulation commands
    Kernel {
        #[command(subcommand)]
        command: KernelCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum BootimgCommand {
    /// Display information about a boot image
    #[command(alias = "i")]
    Info {
        #[arg(short, long)]
        input_boot_file: PathBuf,
    },

    /// Extract components from a boot image
    #[command(alias = "x")]
    Extract {
        #[arg(short, long)]
        input_boot_file: PathBuf,

        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },

    /// Update an existing boot image
    #[command(alias = "u")]
    Update {
        #[arg(short, long, required = true, help = "Boot image file to update")]
        input_boot_file: PathBuf,

        #[arg(short = 'k', long, help = "Kernel file to use for updating")]
        kernel_file: Option<PathBuf>,

        #[arg(short = 'r', long, help = "Ramdisk file to use for updating")]
        ramdisk_file: Option<PathBuf>,

        #[arg(
            short = 's',
            long,
            help = "Second file to use for updating (v0 to v2 only)"
        )]
        second_file: Option<PathBuf>,

        #[arg(long, help = "Recovery DTBO file to use for updating (v1 to v2 only)")]
        recovery_dtbo_file: Option<PathBuf>,

        #[arg(long, help = "DTB file to use for updating (v2 only)")]
        dtb_file: Option<PathBuf>,
    },

    /// Create a new boot image
    Create {
        #[arg(short, long, required = true)]
        output_boot_file: PathBuf,

        #[arg(short, long, required = true)]
        version: u32,

        #[arg(short, long, required = true)]
        kernel_file: PathBuf,

        #[arg(short, long, required = true)]
        ramdisk_file: PathBuf,

        #[arg(short, long)]
        second_file: Option<PathBuf>,

        #[arg(long)]
        recovery_dtbo_file: Option<PathBuf>,

        #[arg(long)]
        dtb_file: Option<PathBuf>,

        #[arg(long, default_value = "2048")]
        page_size: u32,

        #[arg(long, default_value = "0x00008000")]
        kernel_addr: String,

        #[arg(long, default_value = "0x01000000")]
        ramdisk_addr: String,

        #[arg(long, default_value = "0x00000000")]
        second_addr: String,

        #[arg(long, default_value = "0x00000100")]
        tags_addr: String,

        #[arg(long, default_value = "")]
        os_version: String,

        #[arg(long, default_value = "")]
        name: String,

        #[arg(long, default_value = "")]
        cmdline: String,

        #[arg(long, default_value = "")]
        id: String,

        #[arg(long, default_value = "")]
        extra_cmdline: String,

        #[arg(long, default_value = "0")]
        recovery_dtbo_size: u32,

        #[arg(long, default_value = "0x0000000000000000")]
        recovery_dtbo_offset: String,

        #[arg(long, default_value = "0")]
        header_size: u32,

        #[arg(long, default_value = "0")]
        dtb_size: u32,

        #[arg(long, default_value = "0x0000000000000000")]
        dtb_addr: String,

        #[arg(long, default_value = "0")]
        signature_size: u32,
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

fn main() -> Result<(), AbootCrafterError> {
    let cli = Cli::parse();

    match cli.command {
        MainCommand::Bootimg { command } => match command {
            BootimgCommand::Info { input_boot_file } => commands::bootimg::info(&input_boot_file)?,
            BootimgCommand::Extract {
                input_boot_file,
                output_dir,
            } => commands::bootimg::extract(&input_boot_file, output_dir)?,
            BootimgCommand::Update {
                input_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
                recovery_dtbo_file: _,
                dtb_file: _,
            } => {
                commands::bootimg::update(&input_boot_file, kernel_file, ramdisk_file, second_file)?
            }
            BootimgCommand::Create {
                output_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
                recovery_dtbo_file: _,
                dtb_file: _,
                version,
                page_size,
                kernel_addr,
                ramdisk_addr,
                second_addr,
                tags_addr,
                os_version,
                name,
                cmdline,
                id,
                extra_cmdline,
                recovery_dtbo_size,
                recovery_dtbo_offset,
                header_size,
                dtb_size,
                dtb_addr,
                signature_size,
            } => commands::bootimg::create(
                &output_boot_file,
                &kernel_file,
                &ramdisk_file,
                second_file,
                version,
                page_size,
                kernel_addr,
                ramdisk_addr,
                second_addr,
                tags_addr,
                os_version,
                name,
                cmdline,
                id,
                extra_cmdline,
                recovery_dtbo_size,
                recovery_dtbo_offset,
                header_size,
                dtb_size,
                dtb_addr,
                signature_size,
            )?,
        },
        MainCommand::Ramdisk { command } => match command {
            RamdiskCommand::Info { input_file } => unimplemented!(),
            RamdiskCommand::Recompress {
                input_file,
                compression,
            } => unimplemented!(),
            RamdiskCommand::Unpack {
                input_file,
                output_dir,
            } => unimplemented!(),
            RamdiskCommand::Repack {
                input_dir,
                output_file,
                compression,
            } => unimplemented!(),
            RamdiskCommand::AddFile {
                ramdisk_file,
                input_file,
                target_path,
            } => unimplemented!(),
            RamdiskCommand::RemoveFile {
                ramdisk_file,
                target_path,
            } => unimplemented!(),
        },
        MainCommand::Devicetree { command } => match command {
            DevicetreeCommand::Info { input_file } => unimplemented!(),
            DevicetreeCommand::Remove {
                input_file,
                node_path,
            } => unimplemented!(),
            DevicetreeCommand::Add {
                input_file,
                node_path,
                properties,
            } => unimplemented!(),
            DevicetreeCommand::Replace {
                input_file,
                node_path,
                replacement_file,
            } => unimplemented!(),
        },
        MainCommand::Signature { command } => match command {
            SignatureCommand::Info { input_file } => unimplemented!(),
            SignatureCommand::Remove { input_file } => unimplemented!(),
            SignatureCommand::Replace {
                input_file,
                signature_file,
            } => unimplemented!(),
            SignatureCommand::Generate {
                input_file,
                key_file,
                output_file,
            } => unimplemented!(),
        },
        MainCommand::Kernel { command } => match command {
            KernelCommand::Info { input_file } => unimplemented!(),
            KernelCommand::ExtractConfig {
                input_file,
                output_file,
            } => unimplemented!(),
        },
    }

    Ok(())
}
