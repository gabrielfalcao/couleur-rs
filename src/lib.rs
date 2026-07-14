#![allow(unused)]

pub(crate) mod errors;
pub use errors::{ConversionToF32Error, ConversionToU8Error, Error, Exit, Result};

pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub(crate) mod float;
pub use float::{FloatMetadata, leading_zeros_fractional};

pub mod colors;
pub use colors::{
    BLACK, HEX_RGB_REGEX, RGB_COLOR_REGEX, RGBColor, RGBParseError, RGBValue, RgbTriple,
    SINGLE_BAND_DECIMAL_RGB_REGEX, SINGLE_BAND_HEX_RGB_REGEX, TRIPLE_RGB_REGEX, U8Triple, WHITE,
    max_rgb, min_rgb,
};

pub(crate) mod term;
pub use term::{Algorithm, Algorithms, Layer, Reset, Wrap};

pub(crate) mod macros;
// pub use macros::impl_op;
