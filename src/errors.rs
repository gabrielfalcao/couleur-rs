use crate::color::RGBParseError;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};
use winnow::{
    error::{AddContext, ContextError, ErrMode, ParserError, StrContext},
    stream::Stream,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IOError(String),
    RuntimeError(String),
    ConversionToU8Error(String),
    TerminalQueryError(String),
    RenderError(String),
    YamlError(String),
    JsonError(String),
    InitializationError(String),
    ConfigurationError(String),
    ClapError(String),
    ParseError(ParseError),
    TemplateParseError(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Error::IOError(value) => value.to_string(),
                Error::RuntimeError(value) => value.to_string(),
                Error::ConversionToU8Error(value) => value.to_string(),
                Error::TerminalQueryError(value) => value.to_string(),
                Error::RenderError(value) => value.to_string(),
                Error::YamlError(value) => value.to_string(),
                Error::JsonError(value) => value.to_string(),
                Error::InitializationError(value) => value.to_string(),
                Error::ConfigurationError(value) => value.to_string(),
                Error::ClapError(value) => value.to_string(),
                Error::ParseError(value) => value.to_string(),
                Error::TemplateParseError(value) => value.to_string(),
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
            Error::TerminalQueryError(value) => value.to_string(),
            Error::RenderError(value) => value.to_string(),
            Error::YamlError(value) => value.to_string(),
            Error::JsonError(value) => value.to_string(),
            Error::InitializationError(value) => value.to_string(),
            Error::ConfigurationError(value) => value.to_string(),
            Error::ClapError(value) => value.to_string(),
            Error::ParseError(value) => value.to_string(),
            Error::TemplateParseError(value) => value.to_string(),
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
impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::YamlError(e.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e.to_string())
    }
}
impl From<terminal_colorsaurus::Error> for Error {
    fn from(e: terminal_colorsaurus::Error) -> Self {
        Error::TerminalQueryError(e.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseError(Into::<ParseError>::into(e.to_string()))
    }
}
impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Error::ParseError(Into::<ParseError>::into(e.to_string()))
    }
}
impl From<RGBParseError> for Error {
    fn from(e: RGBParseError) -> Self {
        Error::ParseError(Into::<ParseError>::into(e.to_string()))
    }
}
impl From<ErrMode<ContextError>> for Error {
    fn from(e: ErrMode<ContextError>) -> Error {
        Error::ParseError(Into::<ParseError>::into(e.to_string()))
    }
}
// impl<'i, E> From<E> for Error
// where
//     E: ParserError<&'i str> + AddContext<&'i str, StrContext> + ToString,
// {
//     fn from(e: E) -> Error {
//         Error::ParseError(e.to_string().into())
//     }
// }
#[cfg(any(feature = "logging", feature = "tracing"))]
impl From<log::SetLoggerError> for Error {
    fn from(e: log::SetLoggerError) -> Error {
        Error::ConfigurationError(e.to_string())
    }
}

impl ParserError<&str> for Error {
    type Inner = ParseError;

    // Required methods
    fn from_input(input: &&str) -> Self {
        Error::ParseError(input.to_string().into())
    }
    fn into_inner(self) -> std::result::Result<Self::Inner, Self> {
        Err(self.clone())
    }
}
// impl<T> From<T> for Error
// where
//     T: std::error::Error + std::fmt::Display + Sized,
// {
//     fn from(e: T) -> Error {
//         Error::ParseError(Into::<ParseError>::into(e.to_string()))
//     }
// }

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseError {
    message: String,
    input: Option<String>,
    context: Option<String>,
}
impl ParseError {
    pub fn new<T: Display>(message: T) -> ParseError {
        ParseError { message: message.to_string(), input: None, context: None }
    }
    pub fn with_input<T: Display>(mut self, input: T) -> ParseError {
        self.input = Some(input.to_string());
        self
    }
    pub fn with_context<T: Display>(mut self, context: T) -> ParseError {
        self.context = Some(context.to_string());
        self
    }

    /// replaces the `ParseError::input` field with the given text and
    /// returns the previous value of the `input` field.
    pub fn replace_input<T: Display>(&mut self, input: T) -> Option<String> {
        let result = self.input.clone();
        self.input = Some(input.to_string());
        result
    }

    /// replaces the `ParseError::context` field with the given text and
    /// returns the previous value of the `context` field.
    pub fn replace_context<T: Display>(&mut self, context: T) -> Option<String> {
        let result = self.context.clone();
        self.context = Some(context.to_string());
        result
    }
}
impl std::error::Error for ParseError {}
impl ParserError<&str> for ParseError {
    type Inner = ParseError;

    fn from_input(input: &&str) -> Self {
        ParseError::new::<String>(input.to_string().into()).with_input(input.to_string())
    }
    fn into_inner(self) -> std::result::Result<Self::Inner, Self> {
        Err(self.clone())
    }
}
impl<I, C> AddContext<I, C> for ParseError
where
    I: Display + Stream,
    C: Display,
{
    fn add_context(self, input: &I, token_start: &<I as Stream>::Checkpoint, context: C) -> Self {
        self.with_input(input.to_string()).with_context(context.to_string())
    }
}
impl From<String> for ParseError {
    fn from(message: String) -> ParseError {
        ParseError { message, context: None, input: None }
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = &self.message;
        if let Some(context) = &self.context {
            write!(f, "parse error: {message} (context: {context})")
        } else {
            write!(f, "parse error: {message}")
        }
    }
}

/// Contains information of errors which occur while converting RGB band values from [`f32`] to [`u8`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionToU8Error(pub f32, pub String);
impl std::error::Error for ConversionToU8Error {}
impl Display for ConversionToU8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (from_value, message) = (self.0, self.1.to_string());
        write!(f, "Failed to convert {from_value}u8 to f32: {message}")
    }
}

/// Contains information of errors which occur while converting RGB band values from [`u8`] to [`f32`].
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
        Error::ConversionToU8Error(
            ConversionToF32Error(self.0 as u8, format!("cannot convert {} to f32", self.0))
                .to_string(),
        )
    }
}

#[doc(hidden)]
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
