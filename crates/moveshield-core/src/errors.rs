//! MoveShield error handling.

use std::fmt;

/// Standard Result type for MoveShield.
pub type Result<T> = std::result::Result<T, Error>;

/// MoveShield Error representation.
#[derive(Debug)]
pub enum Error {
    /// IO error occurred.
    Io(std::io::Error),
    /// Configuration parse/load error.
    Config(String),
    /// Project discovery or compilation error.
    Compiler(String),
    /// Custom string error.
    Custom(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Config(s) => write!(f, "Configuration error: {}", s),
            Error::Compiler(s) => write!(f, "Compiler/Adapter error: {}", s),
            Error::Custom(s) => write!(f, "Error: {}", s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
