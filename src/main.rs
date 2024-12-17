use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use clap::{Parser, Subcommand};
use thiserror::Error;

const BOOT_MAGIC: &[u8; 8] = b"ANDROID!";
const BOOT_NAME_SIZE: usize = 16;
const BOOT_ARGS_SIZE: usize = 512;

#[derive(Error, Debug)]
enum AbootCrafterError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid magic bytes")]
    InvalidMagicBytes,

    #[error("Invalid boot image: {0}")]
    InvalidImage(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone)]
struct BootImgHeader {
    magic: [u8; 8],
    kernel_size: u32,
    kernel_addr: u32,
    ramdisk_size: u32,
    ramdisk_addr: u32,
    second_size: u32,
    second_addr: u32,
    tags_addr: u32,
    page_size: u32,
    name: [u8; BOOT_NAME_SIZE],
    cmdline: [u8; BOOT_ARGS_SIZE],
    id: [u32; 8],
}

impl Default for BootImgHeader {
    fn default() -> Self {
        Self {
            magic: *BOOT_MAGIC,
            kernel_size: 0,
            kernel_addr: 0,
            ramdisk_size: 0,
            ramdisk_addr: 0,
            second_size: 0,
            second_addr: 0,
            tags_addr: 0,
            page_size: 2048,
            name: [0; BOOT_NAME_SIZE],
            cmdline: [0; BOOT_ARGS_SIZE],
            id: [0; 8],
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct BootConfig {
    boot_size: Option<u64>,
    page_size: Option<u32>,
    kernel_addr: Option<u32>,
    ramdisk_addr: Option<u32>,
    second_addr: Option<u32>,
    tags_addr: Option<u32>,
    cmdline: Option<String>,
}

#[derive(Parser, Debug)]
#[command(about = "Manipulate android boot images like a real blacksmith")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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

fn read_header(file: &mut File) -> Result<BootImgHeader, AbootCrafterError> {
    file.seek(SeekFrom::Start(0))?;

    let mut header = BootImgHeader::default();

    file.read_exact(&mut header.magic)?;
    if header.magic != *BOOT_MAGIC {
        return Err(AbootCrafterError::InvalidMagicBytes);
    }

    header.kernel_size = file.read_u32::<LittleEndian>()?;
    header.kernel_addr = file.read_u32::<LittleEndian>()?;
    header.ramdisk_size = file.read_u32::<LittleEndian>()?;
    header.ramdisk_addr = file.read_u32::<LittleEndian>()?;
    header.second_size = file.read_u32::<LittleEndian>()?;
    header.second_addr = file.read_u32::<LittleEndian>()?;
    header.tags_addr = file.read_u32::<LittleEndian>()?;
    header.page_size = file.read_u32::<LittleEndian>()?;

    file.read_u32::<LittleEndian>()?;
    file.read_u32::<LittleEndian>()?;

    file.read_exact(&mut header.name)?;
    file.read_exact(&mut header.cmdline)?;

    for id in &mut header.id {
        *id = file.read_u32::<LittleEndian>()?;
    }

    Ok(header)
}

fn write_header(file: &mut File, header: &BootImgHeader) -> Result<(), AbootCrafterError> {
    file.seek(SeekFrom::Start(0))?;

    file.write_all(&header.magic)?;
    file.write_u32::<LittleEndian>(header.kernel_size)?;
    file.write_u32::<LittleEndian>(header.kernel_addr)?;
    file.write_u32::<LittleEndian>(header.ramdisk_size)?;
    file.write_u32::<LittleEndian>(header.ramdisk_addr)?;
    file.write_u32::<LittleEndian>(header.second_size)?;
    file.write_u32::<LittleEndian>(header.second_addr)?;
    file.write_u32::<LittleEndian>(header.tags_addr)?;
    file.write_u32::<LittleEndian>(header.page_size)?;
    file.write_u32::<LittleEndian>(0)?;
    file.write_u32::<LittleEndian>(0)?;
    file.write_all(&header.name)?;
    file.write_all(&header.cmdline)?;

    for id in &header.id {
        file.write_u32::<LittleEndian>(*id)?;
    }

    Ok(())
}

fn info(input_boot_file: &PathBuf) -> Result<(), AbootCrafterError> {
    let mut file = File::open(input_boot_file)?;
    let header = read_header(&mut file)?;

    let config = BootConfig {
        boot_size: Some(file.metadata()?.len()),
        page_size: Some(header.page_size),
        kernel_addr: Some(header.kernel_addr),
        ramdisk_addr: Some(header.ramdisk_addr),
        second_addr: Some(header.second_addr),
        tags_addr: Some(header.tags_addr),
        cmdline: Some(
            String::from_utf8_lossy(&header.cmdline)
                .trim_matches('\0')
                .to_string(),
        ),
    };

    // Use toml to pretty print
    let toml_str = toml::to_string_pretty(&config)
        .map_err(|e| AbootCrafterError::ConfigError(e.to_string()))?;

    println!("Android Boot Image Info:");
    println!("File: {}", input_boot_file.display());
    println!("Kernel Size: {} bytes", header.kernel_size);
    println!("Ramdisk Size: {} bytes", header.ramdisk_size);
    println!("Second Stage Size: {} bytes", header.second_size);
    println!("\nConfiguration:");
    print!("{}", toml_str);

    Ok(())
}

fn extract(
    input_boot_file: &PathBuf,
    output_dir: Option<PathBuf>,
) -> Result<(), AbootCrafterError> {
    let mut file = File::open(input_boot_file)?;
    let header = read_header(&mut file)?;

    // Determine the output directory
    let output_dir = output_dir.unwrap_or_else(|| PathBuf::from("out"));
    fs::create_dir_all(&output_dir)?; // Ensure the directory exists

    // Construct file paths
    let config_path = output_dir.join("bootimg.toml");
    let kernel_path = output_dir.join("kernel.img");
    let ramdisk_path = output_dir.join("ramdisk.img");
    let second_path = output_dir.join("second.img");

    // Prepare config
    let config = BootConfig {
        boot_size: Some(file.metadata()?.len()),
        page_size: Some(header.page_size),
        kernel_addr: Some(header.kernel_addr),
        ramdisk_addr: Some(header.ramdisk_addr),
        second_addr: Some(header.second_addr),
        tags_addr: Some(header.tags_addr),
        cmdline: Some(
            String::from_utf8_lossy(&header.cmdline)
                .trim_matches('\0')
                .to_string(),
        ),
    };

    // Write config
    let toml_str = toml::to_string_pretty(&config)
        .map_err(|e| AbootCrafterError::ConfigError(e.to_string()))?;
    fs::write(&config_path, toml_str)?;

    // Extract kernel
    if header.kernel_size > 0 {
        let mut kernel_buf = vec![0; header.kernel_size as usize];
        file.seek(SeekFrom::Start(header.page_size as u64))?;
        file.read_exact(&mut kernel_buf)?;
        fs::write(&kernel_path, kernel_buf)?;
    }

    // Extract ramdisk
    if header.ramdisk_size > 0 {
        let mut ramdisk_buf = vec![0; header.ramdisk_size as usize];
        let ramdisk_offset = (1 + header.kernel_size.div_ceil(header.page_size)) * header.page_size;
        file.seek(SeekFrom::Start(ramdisk_offset as u64))?;
        file.read_exact(&mut ramdisk_buf)?;
        fs::write(&ramdisk_path, ramdisk_buf)?;
    }

    // Extract second stage
    if header.second_size > 0 {
        let mut second_buf = vec![0; header.second_size as usize];
        let second_offset = (1
            + header.kernel_size.div_ceil(header.page_size)
            + header.ramdisk_size.div_ceil(header.page_size))
            * header.page_size;
        file.seek(SeekFrom::Start(second_offset as u64))?;
        file.read_exact(&mut second_buf)?;
        fs::write(&second_path, second_buf)?;
    }

    Ok(())
}

fn update(
    input_boot_file: &PathBuf,
    config_file: Option<PathBuf>,
    kernel_file: Option<PathBuf>,
    ramdisk_file: Option<PathBuf>,
    second_file: Option<PathBuf>,
    cmdline_params: &[String],
) -> Result<(), AbootCrafterError> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(input_boot_file)?;
    let mut header = read_header(&mut file)?;

    // Update from config file
    if let Some(cfg_path) = config_file {
        let cfg_content = fs::read_to_string(&cfg_path)?;
        let config: BootConfig = toml::from_str(&cfg_content)
            .map_err(|e| AbootCrafterError::ConfigError(e.to_string()))?;

        if let Some(kernel_addr) = config.kernel_addr {
            header.kernel_addr = kernel_addr;
        }
        if let Some(ramdisk_addr) = config.ramdisk_addr {
            header.ramdisk_addr = ramdisk_addr;
        }
        if let Some(second_addr) = config.second_addr {
            header.second_addr = second_addr;
        }
        if let Some(tags_addr) = config.tags_addr {
            header.tags_addr = tags_addr;
        }
        if let Some(page_size) = config.page_size {
            header.page_size = page_size;
        }
        if let Some(cmdline) = config.cmdline {
            let bytes = cmdline.as_bytes();
            header.cmdline[..bytes.len().min(BOOT_ARGS_SIZE)]
                .copy_from_slice(&bytes[..bytes.len().min(BOOT_ARGS_SIZE)]);
        }
    }

    // Update from command line parameters
    for param in cmdline_params {
        if let Some((key, value)) = param.split_once('=') {
            if key.trim() == "cmdline" {
                let trimmed = value.trim();
                let bytes = trimmed.as_bytes();
                header.cmdline[..bytes.len().min(BOOT_ARGS_SIZE)]
                    .copy_from_slice(&bytes[..bytes.len().min(BOOT_ARGS_SIZE)]);
            }
        }
    }

    // Update kernel
    if let Some(kernel_path) = kernel_file {
        let kernel_data = fs::read(&kernel_path)?;
        header.kernel_size = kernel_data
            .len()
            .try_into()
            .map_err(|_| AbootCrafterError::InvalidImage("Kernel too large".to_string()))?;
        file.seek(SeekFrom::Start(header.page_size as u64))?;
        file.write_all(&kernel_data)?;
    }

    // Update ramdisk
    if let Some(ramdisk_path) = ramdisk_file {
        let ramdisk_data = fs::read(&ramdisk_path)?;
        header.ramdisk_size = ramdisk_data
            .len()
            .try_into()
            .map_err(|_| AbootCrafterError::InvalidImage("Ramdisk too large".to_string()))?;
        let ramdisk_offset = (1 + header.kernel_size.div_ceil(header.page_size)) * header.page_size;
        file.seek(SeekFrom::Start(ramdisk_offset as u64))?;
        file.write_all(&ramdisk_data)?;
    }

    // Update second stage
    if let Some(second_path) = second_file {
        let second_data = fs::read(&second_path)?;
        header.second_size = second_data
            .len()
            .try_into()
            .map_err(|_| AbootCrafterError::InvalidImage("Second stage too large".to_string()))?;
        let second_offset = (1
            + header.kernel_size.div_ceil(header.page_size)
            + header.ramdisk_size.div_ceil(header.page_size))
            * header.page_size;
        file.seek(SeekFrom::Start(second_offset as u64))?;
        file.write_all(&second_data)?;
    }

    // Write the updated header
    write_header(&mut file, &header)?;

    Ok(())
}

fn create(
    output_boot_img: &PathBuf,
    kernel_file: &PathBuf,
    ramdisk_file: &PathBuf,
    second_file: Option<PathBuf>,
    config_file: Option<PathBuf>,
    cmdline_params: &[String],
) -> Result<(), AbootCrafterError> {
    let mut header = BootImgHeader::default();

    // Read kernel
    let kernel_data = fs::read(kernel_file)?;
    header.kernel_size = kernel_data
        .len()
        .try_into()
        .map_err(|_| AbootCrafterError::InvalidImage("Kernel too large".to_string()))?;

    // Read ramdisk
    let ramdisk_data = fs::read(ramdisk_file)?;
    header.ramdisk_size = ramdisk_data
        .len()
        .try_into()
        .map_err(|_| AbootCrafterError::InvalidImage("Ramdisk too large".to_string()))?;

    // Read second stage if provided
    let second_data = second_file.map(fs::read).transpose()?;
    if let Some(ref data) = second_data {
        header.second_size = data
            .len()
            .try_into()
            .map_err(|_| AbootCrafterError::InvalidImage("Second stage too large".to_string()))?;
    }

    // Process configuration if provided
    if let Some(cfg_path) = config_file {
        let cfg_content = fs::read_to_string(&cfg_path)?;
        let config: BootConfig = toml::from_str(&cfg_content)
            .map_err(|e| AbootCrafterError::ConfigError(e.to_string()))?;

        if let Some(kernel_addr) = config.kernel_addr {
            header.kernel_addr = kernel_addr;
        }
        if let Some(ramdisk_addr) = config.ramdisk_addr {
            header.ramdisk_addr = ramdisk_addr;
        }
        if let Some(second_addr) = config.second_addr {
            header.second_addr = second_addr;
        }
        if let Some(tags_addr) = config.tags_addr {
            header.tags_addr = tags_addr;
        }
        if let Some(page_size) = config.page_size {
            header.page_size = page_size;
        }
        if let Some(cmdline) = config.cmdline {
            let bytes = cmdline.as_bytes();
            header.cmdline[..bytes.len().min(BOOT_ARGS_SIZE)]
                .copy_from_slice(&bytes[..bytes.len().min(BOOT_ARGS_SIZE)]);
        }
    }

    // Process command line parameters
    for param in cmdline_params {
        if let Some((key, value)) = param.split_once('=') {
            if key.trim() == "cmdline" {
                let trimmed = value.trim();
                let bytes = trimmed.as_bytes();
                header.cmdline[..bytes.len().min(BOOT_ARGS_SIZE)]
                    .copy_from_slice(&bytes[..bytes.len().min(BOOT_ARGS_SIZE)]);
            }
        }
    }

    // Create boot image file
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_boot_img)?;

    // Write header
    write_header(&mut file, &header)?;

    // Write kernel
    file.seek(SeekFrom::Start(header.page_size as u64))?;
    file.write_all(&kernel_data)?;

    // Pad kernel
    let kernel_pages = header.kernel_size.div_ceil(header.page_size);
    let kernel_padding = kernel_pages * header.page_size - header.kernel_size;
    file.write_all(&vec![0; kernel_padding as usize])?;

    // Write ramdisk
    let ramdisk_offset = (1 + kernel_pages) * header.page_size;
    file.seek(SeekFrom::Start(ramdisk_offset as u64))?;
    file.write_all(&ramdisk_data)?;

    // Pad ramdisk
    let ramdisk_pages = header.ramdisk_size.div_ceil(header.page_size);
    let ramdisk_padding = ramdisk_pages * header.page_size - header.ramdisk_size;
    file.write_all(&vec![0; ramdisk_padding as usize])?;

    // Write second stage if provided
    if let Some(second_data) = second_data {
        let second_offset = (1 + kernel_pages + ramdisk_pages) * header.page_size;
        file.seek(SeekFrom::Start(second_offset as u64))?;
        file.write_all(&second_data)?;

        // Pad second stage
        let second_pages = header.second_size.div_ceil(header.page_size);
        let second_padding = second_pages * header.page_size - header.second_size;
        file.write_all(&vec![0; second_padding as usize])?;
    }

    Ok(())
}

fn main() -> Result<(), AbootCrafterError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { input_boot_file } => info(&input_boot_file)?,
        Commands::Extract {
            input_boot_file,
            output_dir,
        } => extract(&input_boot_file, output_dir)?,
        Commands::Update {
            input_boot_file,
            config_file,
            kernel_file,
            ramdisk_file,
            second_file,
            cmdline_params,
        } => update(
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
        } => create(
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
