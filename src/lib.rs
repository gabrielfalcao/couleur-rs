#![allow(unused)]
//! couleur-rs: parse, apply contrast algorithms to ANSI RGB colors and print text powered by a simple template language
//!
//! ## Parsing Colors
//!
//! ```rust
//! let dark_pink = "#C32454".parse::<Color>()?;
//! let darkest_pink = "#831C5D".parse::<Color>()?;
//! let light_pink = "#FCA790".parse::<Color>()?;
//! let lightest_pink = "#FDCBB0".parse::<Color>()?;
//! ```
//!
//! ## Applying Contrast
//! ```rust
//! let dark_pink = "#C32454".parse::<Color>()?;
//! let darkest_pink = "#831C5D".parse::<Color>()?;
//! let light_pink = "#FCA790".parse::<Color>()?;
//! let lightest_pink = "#FDCBB0".parse::<Color>()?;
//!
//! assert_eq!(
//!     dark_pink.to_triple(),
use std::sync::LazyLock;

use lazy_mut::LazyMut;
#[doc(hidden)] pub mod util;
#[doc(hidden)]
pub use util::{
    HEX_RGB_REGEX,
    SINGLE_BAND_DECIMAL_RGB_REGEX,
    SINGLE_BAND_HEX_RGB_REGEX,
    TRIPLE_RGB_REGEX,
    deserialize_string_to_str,
    serialize_static_str_to_string,
};

#[doc(hidden)] pub mod ansi_colorizer;
#[doc(inline)] pub use ansi_colorizer::AnsiColorizer;

#[doc(hidden)] pub mod to_ansi;
#[doc(inline)] pub use to_ansi::ToAnsi;

#[doc(hidden)] pub mod templating;
#[doc(inline)] pub use templating::{REGEX, render};

#[doc(hidden)] pub mod errors;
#[doc(inline)]
pub use errors::{ConversionToF32Error, ConversionToU8Error, Error, Exit, ParseError, Result};

#[doc(hidden)] pub mod layer;
#[doc(hidden)] pub mod macros;
#[doc(inline)] pub use layer::{Layer, REGEX_LAYER};

#[doc(hidden)] pub mod prefix;
#[doc(inline)] pub use prefix::{Prefix, PrefixContainer};

#[doc(hidden)] pub mod wrap;
#[doc(inline)] pub use wrap::Wrap;

#[doc(hidden)] pub mod cmp;
#[doc(inline)] pub use cmp::{max_rgb, min_rgb};

#[doc(hidden)] pub mod contrast;
#[doc(inline)] pub use contrast::Contrast;

#[doc(hidden)] pub mod terminal;
#[doc(inline)] pub use terminal::{Terminal, TerminalInfo, TerminalInfoError};

// #[cfg(any(feature = "tracing", feature = "logging"))]
#[doc(hidden)] pub mod testing;
#[doc(hidden)] pub use testing::global_setup;

#[doc(hidden)] pub mod traits;
#[doc(inline)] pub use traits::{AnsiRenderable, ToAnsiEscSuffix};

#[doc(hidden)] pub mod renderable_color;
#[doc(inline)] pub use renderable_color::RenderableColor;

#[doc(hidden)] pub mod cli;
#[doc(inline)] pub use cli::RenderingOptions;

#[doc(hidden)] pub mod dispatch;
#[doc(inline)]
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[doc(hidden)] pub mod logging;
#[doc(inline)]
pub use logging::{get_log_path, local_data_dir, setup_logging, setup_tracing};

#[doc(hidden)] pub mod float;
#[doc(inline)]
pub use float::{FloatMetadata, leading_zeros_exp, leading_zeros_fractional};

#[doc(hidden)] pub mod color;
#[doc(inline)]
pub use color::{
    BLACK,
    Color,
    RGBParseError,
    RgbTriple,
    U8Triple,
    WHITE,
    convert_ansi256_to_rgb_triple,
};

pub static TERMINAL: LazyLock<TerminalInfo> = LazyLock::new(|| Terminal::info());
pub static PREFIX: LazyMut<PrefixContainer> = LazyMut::new(|| PrefixContainer::default());

pub fn set_runtime_prefix<T: Into<Prefix>>(prefix: T) {
    let mut current = PREFIX.get_mut();
    current.set(prefix.into());
}

pub fn get_runtime_prefix() -> Prefix {
    crate::PREFIX.get_mut().get()
}

#[doc(hidden)] pub mod value;
#[doc(inline)] pub use value::Value;

#[doc(hidden)] pub mod reset;
#[doc(inline)] pub use reset::Reset;
