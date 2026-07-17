#![allow(unused)]
//!
//! ```rust
//! let dark_pink = "#C32454".parse::<Color>()?;
//! let darkest_pink = "#831C5D".parse::<Color>()?;
//! let light_pink = "#FCA790".parse::<Color>()?;
//! let lightest_pink = "#FDCBB0".parse::<Color>()?;
//!
//! assert_eq!(
//!     dark_pink.to_triple(),
//!     (
//!         Value::from_u8(0xC3)?,
//!         Value::from_u8(0x24)?,
//!         Value::from_u8(0x54)?
//!     )
//! );
//!
//! assert_eq!(lightest_pink.get_adobe_complementary().to_hex_string(), "#B0E2FD");
//! assert_eq!(lightest_pink.get_accessible_contrast().to_hex_string(), "#000000");
//! assert_eq!(lightest_pink.get_binary_contrast().to_hex_string(), "#000000");
//! assert_eq!(lightest_pink.get_msb_invert_contrast().to_hex_string(), "#7D4B30");
//!
//! assert_eq!(darkest_pink.get_adobe_complementary().to_hex_string(), "#1C8342");
//! assert_eq!(darkest_pink.get_accessible_contrast().to_hex_string(), "#000000");
//! assert_eq!(darkest_pink.get_binary_contrast().to_hex_string(), "#FFFFFF");
//! assert_eq!(darkest_pink.get_msb_invert_contrast().to_hex_string(), "#039CDD");
//! ```
//!
pub mod errors;
pub use errors::{ConversionToF32Error, ConversionToU8Error, Error, Exit, Result};

pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub mod float;
pub use float::{FloatMetadata, leading_zeros_fractional};

pub mod color;
pub use color::{BLACK, Color, RGBParseError, WHITE};

pub mod value;
pub use value::Value;

pub mod wrap;
pub use wrap::Wrap;

pub mod reset;
pub use reset::Reset;

pub mod algorithm;
pub use algorithm::Algorithm;

pub mod triples;
pub use triples::{RgbTriple, U8Triple};

pub mod macros;
// pub use macros::impl_op;

pub mod colorize;
pub use colorize::Colorizer;

pub mod layer;
pub use layer::Layer;

pub mod cmp;
pub use cmp::{max_rgb, min_rgb};

pub(crate) mod util;
pub use util::{
    HEX_RGB_REGEX, RESET, SINGLE_BAND_DECIMAL_RGB_REGEX, SINGLE_BAND_HEX_RGB_REGEX,
    TRIPLE_RGB_REGEX,
};
