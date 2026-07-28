use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};
use thiserror::Error as ThisError;

use crate::color::RGBParseError;
#[derive(ThisError, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    #[error("I/O Error")]
    IOError(#[from] std::io::Error),
    #[error("I/O Core Error")]
    IOCoreError(#[from] iocore::Error),
    #[error("Runtime Error {0}")]
    RuntimeError(&'static str),
    #[error("Error converting {0} to u8: {1}")]
    ConversionToU8Error(f32, ConversionToU8Error),
    #[error("Error querying terminal colors: {0}")]
    TerminalQueryError(#[from] terminal_colorsaurus::Error),
    #[error("Render Error: {0}")]
    RenderError(RenderError),
    #[error("Parse Error: {0}")]
    ParseError(&'static str),
    #[error("error parsing integer: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("error parsing float: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("error parsing RGB color: {0}")]
    RGBParseError(#[from] RGBParseError),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConversionToU8Error {
    OutOfBoundary,
}
impl Display for ConversionToU8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConversionToU8Error::OutOfBoundary => "value is out of boundary (0 to 255)",
            }
        )
    }
}
impl std::error::Error for ConversionToU8Error {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderError {
    MissingColors,
}
impl Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RenderError::MissingColors => "neither background nor foreground colors provided",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseError {
    OutOfRange(&'static str),
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParseError::OutOfRange(desc) => format!("value '{desc}' is out of range"),
            }
        )
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionToF32Error(pub u8, pub &'static str);
impl std::error::Error for ConversionToF32Error {}

impl Display for ConversionToF32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (from_value, message) = (self.0, self.1.to_string());
        write!(f, "Failed to convert {from_value}f32 to u8: {message}")
    }
}
impl Into<Error> for ConversionToF32Error {
    fn into(self) -> Error {
        Error::ConversionToU8Error(
            ConversionToF32Error(self.0 as u8, format!("cannot convert {} to f32", self.0))
                .to_string(),
        )
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
