//!
//! ```rust
//! let dark_pink = "#C32454".parse::<RGBColor>()?;
//! let darkest_pink = "#831C5D".parse::<RGBColor>()?;
//! let light_pink = "#FCA790".parse::<RGBColor>()?;
//! let lightest_pink = "#FDCBB0".parse::<RGBColor>()?;
//!
//! assert_eq!(
//!     dark_pink.to_triple(),
//!     (
//!         RGBValue::from_u8(0xC3)?,
//!         RGBValue::from_u8(0x24)?,
//!         RGBValue::from_u8(0x54)?
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
pub use term::{Algorithm, Layer, Reset, Wrap};

pub(crate) mod macros;
// pub use macros::impl_op;
