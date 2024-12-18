use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about = "Manipulate android boot images like a real blacksmith")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
        #[arg(short, long)]
        input_boot_file: PathBuf,

        #[arg(short = 'C', long)]
        config_file: Option<PathBuf>,

        #[arg(short, long)]
        kernel_file: Option<PathBuf>,

        #[arg(short, long)]
        ramdisk_file: Option<PathBuf>,

        #[arg(short, long)]
        second_file: Option<PathBuf>,

        #[arg(short, long)]
        cmdline_params: Vec<String>,
    },

    /// Create a new boot image
    Create {
        #[arg(short, long)]
        output_boot_file: PathBuf,

        #[arg(short, long, required = true)]
        kernel_file: PathBuf,

        #[arg(short, long, required = true)]
        ramdisk_file: PathBuf,

        #[arg(short, long)]
        second_file: Option<PathBuf>,

        #[arg(short = 'C', long, required = true)]
        config_file: Option<PathBuf>,

        #[arg(short, long)]
        cmdline_params: Vec<String>,
    },
}
