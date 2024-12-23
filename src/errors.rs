use thiserror::Error;

#[derive(Error, Debug)]
pub enum AbootCrafterError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
