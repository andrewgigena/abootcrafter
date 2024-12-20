use thiserror::Error;

#[derive(Error, Debug)]
pub enum AbootCrafterError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid header magic")]
    InvalidHeaderMagic,

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
