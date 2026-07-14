use std::fmt::Display;
use std::num::ParseIntError;
use serde::{Deserialize, Serialize};
use crate::colors::RGBParseError;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IOError(String),
    RuntimeError(String),
    ConversionToU8Error(String),

    ClapError(String),
    ParseError(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Error::IOError(value) => value,
                Error::RuntimeError(value) => value,
                Error::ConversionToU8Error(value) => value,

                Error::ClapError(value) => value,
                Error::ParseError(value) => value,
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(value) => value.to_string(),
            Error::RuntimeError(value) => value.to_string(),
            Error::ConversionToU8Error(value) => value.to_string(),

            Error::ClapError(value) => value.to_string(),
            Error::ParseError(value) => value.to_string(),
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.to_string())
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::IOError(e.to_string())
    }
}
impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseError(e.to_string())
    }
}
impl From<RGBParseError> for Error {
    fn from(e: RGBParseError) -> Self {
        Error::ParseError(e.to_string())
    }
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionToU8Error(pub f32, pub String);
impl std::error::Error for ConversionToU8Error {}
impl Display for ConversionToU8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (from_value, message) = (self.0, self.1.to_string());
        write!(f, "Failed to convert {from_value}u8 to f32: {message}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionToF32Error(pub u8, pub String);
impl std::error::Error for ConversionToF32Error {}

impl Display for ConversionToF32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (from_value, message) = (self.0, self.1.to_string());
        write!(f, "Failed to convert {from_value}f32 to u8: {message}")
    }
}
impl Into<Error> for ConversionToF32Error {
    fn into(self) -> Error {
        Error::ConversionToU8Error(ConversionToF32Error(self.0 as u8, format!("cannot convert {} to f32", self.0)).to_string())
    }
}
#[derive(Debug, Clone)]
pub enum Exit {
    Success,
    Error(Error),
}
impl std::process::Termination for Exit {
    fn report(self) -> std::process::ExitCode {
        match &self {
            Exit::Success => std::process::ExitCode::from(0),
            Exit::Error(error) => {
                eprintln!("{}", error);
                std::process::ExitCode::from(1)
            }
        }
    }
}
impl<T> From<std::result::Result<T, Error>> for Exit {
    fn from(result: std::result::Result<T, Error>) -> Exit {
        match result {
            Ok(_) => Exit::Success,
            Err(e) => Exit::Error(e),
        }
    }
}
