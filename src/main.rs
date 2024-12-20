mod cli;
mod commands;
mod errors;
mod headers;

use clap::Parser;
use cli::{Cli, Commands};
use errors::AbootCrafterError;

fn main() -> Result<(), AbootCrafterError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { input_boot_file } => commands::info(&input_boot_file)?,
        Commands::Extract {
            input_boot_file,
            output_dir,
        } => commands::extract(&input_boot_file, output_dir)?,
        Commands::Update {
            input_boot_file,
            config_file,
            kernel_file,
            ramdisk_file,
            second_file,
            cmdline_params,
        } => commands::update(
            &input_boot_file,
            config_file,
            kernel_file,
            ramdisk_file,
            second_file,
            &cmdline_params,
        )?,
        Commands::Create {
            output_boot_file,
            kernel_file,
            ramdisk_file,
            second_file,
            config_file,
            cmdline_params,
        } => commands::create(
            &output_boot_file,
            &kernel_file,
            &ramdisk_file,
            second_file,
            config_file,
            &cmdline_params,
        )?,
    }

    Ok(())
}
