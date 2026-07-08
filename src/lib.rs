#![allow(unused)]

pub(crate) mod errors;
pub use errors::{ConversionToF32Error, ConversionToU8Error, Error, Exit, Result};

pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub(crate) mod float;
pub use float::{FloatMetadata, leading_zeros_fractional};

pub(crate) mod colors;
pub use colors::{
    HEX_RGB_REGEX, RGBValue, SINGLE_BAND_DECIMAL_RGB_REGEX, SINGLE_BAND_HEX_RGB_REGEX,
    TRIPLE_RGB_REGEX,
};

pub(crate) mod macros;
// pub use macros::impl_op;
