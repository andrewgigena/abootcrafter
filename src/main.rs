#![forbid(unsafe_code)]
mod cli;
mod commands;
mod errors;
mod headers;

use clap::Parser;
use cli::{Cli, CreateCommand, ExtractCommand, InfoCommand, MainCommand, UpdateCommand};
use errors::AbootCrafterError;

fn main() -> Result<(), AbootCrafterError> {
    let cli = Cli::parse();

    match cli.command {
        MainCommand::Info { command } => match command {
            InfoCommand::Bootimg { input_boot_file } => commands::info::info(&input_boot_file)?,
        },
        MainCommand::Extract { command } => match command {
            ExtractCommand::Bootimg {
                input_boot_file,
                output_dir,
            } => commands::extract::extract(&input_boot_file, output_dir)?,
        },
        MainCommand::Update { command } => match command {
            UpdateCommand::Bootimg {
                input_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
                recovery_dtbo_file,
                dtb_file,
                cmdline,
                extra_cmdline,
            } => commands::update::update(
                &input_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
                recovery_dtbo_file,
                dtb_file,
                cmdline,
                extra_cmdline,
            )?,
        },
        MainCommand::Create { command } => match command {
            CreateCommand::BootimgV0 {
                output_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
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
            } => commands::create::create_v0(
                output_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
                page_size as u32,
                kernel_addr,
                ramdisk_addr,
                second_addr,
                tags_addr,
                os_version,
                name,
                cmdline,
                id,
                extra_cmdline,
            )?,
            CreateCommand::BootimgV1 {
                output_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
                recovery_dtbo_file,
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
                recovery_dtbo_offset,
            } => commands::create::create_v1(
                output_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
                recovery_dtbo_file,
                page_size as u32,
                kernel_addr,
                ramdisk_addr,
                second_addr,
                tags_addr,
                os_version,
                name,
                cmdline,
                id,
                extra_cmdline,
                recovery_dtbo_offset,
            )?,
            CreateCommand::BootimgV2 {
                output_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
                recovery_dtbo_file,
                dtb_file,
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
                recovery_dtbo_offset,
                dtb_addr,
            } => commands::create::create_v2(
                output_boot_file,
                kernel_file,
                ramdisk_file,
                second_file,
                recovery_dtbo_file,
                dtb_file,
                page_size as u32,
                kernel_addr,
                ramdisk_addr,
                second_addr,
                tags_addr,
                os_version,
                name,
                cmdline,
                id,
                extra_cmdline,
                recovery_dtbo_offset,
                dtb_addr,
            )?,
            CreateCommand::BootimgV3 {
                output_boot_file,
                kernel_file,
                ramdisk_file,
                os_version,
                cmdline,
            } => commands::create::create_v3(
                output_boot_file,
                kernel_file,
                ramdisk_file,
                os_version,
                cmdline,
            )?,
            CreateCommand::BootimgV4 {
                output_boot_file,
                kernel_file,
                ramdisk_file,
                os_version,
                cmdline,
            } => commands::create::create_v4(
                output_boot_file,
                kernel_file,
                ramdisk_file,
                os_version,
                cmdline,
            )?,
        },
        // MainCommand::Ramdisk { command } => match command {
        //     RamdiskCommand::Info { input_file: _ } => unimplemented!(),
        //     RamdiskCommand::Recompress {
        //         input_file: _,
        //         compression: _,
        //     } => unimplemented!(),
        //     RamdiskCommand::Unpack {
        //         input_file: _,
        //         output_dir: _,
        //     } => unimplemented!(),
        //     RamdiskCommand::Repack {
        //         input_dir: _,
        //         output_file: _,
        //         compression: _,
        //     } => unimplemented!(),
        //     RamdiskCommand::AddFile {
        //         ramdisk_file: _,
        //         input_file: _,
        //         target_path: _,
        //     } => unimplemented!(),
        //     RamdiskCommand::RemoveFile {
        //         ramdisk_file: _,
        //         target_path: _,
        //     } => unimplemented!(),
        // },
        // MainCommand::Devicetree { command } => match command {
        //     DevicetreeCommand::Info { input_file: _ } => unimplemented!(),
        //     DevicetreeCommand::Remove {
        //         input_file: _,
        //         node_path: _,
        //     } => unimplemented!(),
        //     DevicetreeCommand::Add {
        //         input_file: _,
        //         node_path: _,
        //         properties: _,
        //     } => unimplemented!(),
        //     DevicetreeCommand::Replace {
        //         input_file: _,
        //         node_path: _,
        //         replacement_file: _,
        //     } => unimplemented!(),
        // },
        // MainCommand::Signature { command } => match command {
        //     SignatureCommand::Info { input_file: _ } => unimplemented!(),
        //     SignatureCommand::Remove { input_file: _ } => unimplemented!(),
        //     SignatureCommand::Replace {
        //         input_file: _,
        //         signature_file: _,
        //     } => unimplemented!(),
        //     SignatureCommand::Generate {
        //         input_file: _,
        //         key_file: _,
        //         output_file: _,
        //     } => unimplemented!(),
        // },
        // MainCommand::Kernel { command } => match command {
        //     KernelCommand::Info { input_file: _ } => unimplemented!(),
        //     KernelCommand::ExtractConfig {
        //         input_file: _,
        //         output_file: _,
        //     } => unimplemented!(),
        // },
    }

    Ok(())
}
