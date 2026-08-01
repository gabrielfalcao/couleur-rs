use std::fmt::Display;

use serde::{Deserialize, Serialize};
use from_pest::{Void, ConversionError};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    IOError(String),
    ConfigError(String),
    SafetyError(String),
    ParseError(String),
    SyntaxTableError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Error::IOError(s) => s.to_string(),
                Error::ConfigError(s) => s.to_string(),
                Error::SafetyError(s) => s.to_string(),
                Error::ParseError(s) => s.to_string(),
                Error::SyntaxTableError(s) => s.to_string(),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(_) => "IOError",
            Error::ConfigError(_) => "ConfigError",
            Error::SafetyError(_) => "SafetyError",
            Error::ParseError(_) => "ParseError",
            Error::SyntaxTableError(_) => "SyntaxTableError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::ConfigError(format!("{}", e))
    }
}
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::ConfigError(format!("{}", e))
    }
}
impl From<sanitation::Error<'_>> for Error {
    fn from(e: sanitation::Error<'_>) -> Self {
        Error::SafetyError(format!("{}", e))
    }
}
impl From<ConversionError<Void>> for Error {
    fn from(e: ConversionError<Void>) -> Self {
        Error::ParseError(e.to_string())
    }
}
pub type Result<T> = std::result::Result<T, Error>;
