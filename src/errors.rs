use thiserror::Error;

#[derive(Error, Debug)]
pub enum AbootCrafterError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid magic bytes")]
    InvalidMagicBytes,

    #[error("Invalid boot image: {0}")]
    InvalidImage(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
