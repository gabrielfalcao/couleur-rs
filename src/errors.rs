use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    IOError(String),
    RuntimeError(String),

    ClapError(String),

    ColorEyreError(String),

    HeckError(String),

    HexError(String),

    IocoreError(String),

    OgEnumToStringError(String),

    OwoColorsError(String),

    RegexError(String),

    Rgb2ansi256Error(String),

    SerdeError(String),

    SupportsColorError(String),

    ThiserrorError(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Error::IOError(e) => e.to_string(),
                Error::RuntimeError(e) => e.to_string(),

                Error::ClapError(e) => e.to_string(),

                Error::ColorEyreError(e) => e.to_string(),

                Error::HeckError(e) => e.to_string(),

                Error::HexError(e) => e.to_string(),

                Error::IocoreError(e) => e.to_string(),

                Error::OgEnumToStringError(e) => e.to_string(),

                Error::OwoColorsError(e) => e.to_string(),

                Error::RegexError(e) => e.to_string(),

                Error::Rgb2ansi256Error(e) => e.to_string(),

                Error::SerdeError(e) => e.to_string(),

                Error::SupportsColorError(e) => e.to_string(),

                Error::ThiserrorError(e) => e.to_string(),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(_) => "IOError",
            Error::RuntimeError(_) => "RuntimeError",

            Error::ClapError(_) => "ClapError",

            Error::ColorEyreError(_) => "ColorEyreError",

            Error::HeckError(_) => "HeckError",

            Error::HexError(_) => "HexError",

            Error::IocoreError(_) => "IocoreError",

            Error::OgEnumToStringError(_) => "OgEnumToStringError",

            Error::OwoColorsError(_) => "OwoColorsError",

            Error::RegexError(_) => "RegexError",

            Error::Rgb2ansi256Error(_) => "Rgb2ansi256Error",

            Error::SerdeError(_) => "SerdeError",

            Error::SupportsColorError(_) => "SupportsColorError",

            Error::ThiserrorError(_) => "ThiserrorError",
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
pub type Result<T> = std::result::Result<T, Error>;

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
