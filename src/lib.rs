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
//!     (Value::from_u8(0xC3)?, Value::from_u8(0x24)?, Value::from_u8(0x54)?)
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
//! ## Template Language
//! ```rust
//! parse::<&str, ContextError>("{color:#1EBC73}Hello{color:#1EBC73%contrast:web} World");
//! Ok(Node::Array(vec![
//!     Node::Color("#1EBC73".parse::<couleur_rs::Color>().unwrap()),
//!     Node::Text("Hello".to_string()),
//!     Node::RenderableColor(
//!         RenderableColor::new("#1EBC73".parse::<couleur_rs::Color>().unwrap())
//!             .with_contrast(Contrast::Web)
//!     ),
//!     Node::Text(" World".to_string())
//! ]))
//! ```
//! 
//!
//! <div class="note">
//! <h1>NOTE</h1>
//!
//! This API can cause undefined behavior if the bounds checking is
//! bypassed.  This library is currently a work-in-progress and
//! provides the feature flags `logging` and `tracing` as debugging
//! tools. All the tracing and logging annotations and calls will be
//! removed before `couleur-rs` reaches version 1.0.0
//!
//! </div>
use std::sync::LazyLock;
#[doc(hidden)] pub mod ansi_colorizer;
#[doc(inline)] pub use ansi_colorizer::AnsiColorizer;
#[doc(hidden)] pub mod cmp;
#[doc(inline)] pub use cmp::{max_rgb, min_rgb};
#[doc(hidden)] pub mod color;
#[doc(inline)] pub use color::{BLACK, Color, RGBParseError, RgbTriple, U8Triple, WHITE};
#[doc(hidden)] pub mod contrast;
#[doc(inline)] pub use contrast::Contrast;
#[doc(hidden)] pub mod dispatch;
#[doc(inline)] pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
#[doc(hidden)] pub mod errors;
#[doc(hidden)] pub use errors::Exit;
#[doc(inline)]
pub use errors::{ConversionToF32Error, ConversionToU8Error, Error, ParseError, Result};
#[doc(hidden)] pub mod float;
#[doc(inline)] pub use float::{FloatMetadata, leading_zeros_exp, leading_zeros_fractional};
#[doc(hidden)] pub mod layer;
#[doc(inline)] pub use layer::Layer;
#[doc(hidden)] pub mod macros;
#[doc(hidden)] pub mod prefix;
#[doc(inline)] pub use prefix::Prefix;
#[doc(hidden)] pub mod reset;
#[doc(inline)] pub use reset::Reset;
#[doc(hidden)] pub mod terminal;
#[doc(inline)] pub use terminal::{Terminal, TerminalInfo, TerminalInfoError};
#[doc(hidden)] pub mod state;
#[doc(inline)] pub use state::{ColorPalette, Context};
#[doc(hidden)] pub mod value;
#[doc(inline)] pub use value::Value;
#[doc(hidden)] pub mod wrap;
#[doc(inline)] pub use wrap::Wrap;
#[doc(hidden)] pub mod renderable_color;
#[doc(inline)] pub use renderable_color::RenderableColor;
#[doc(hidden)] pub mod traits;
#[doc(inline)] pub use traits::ToAnsiEscSuffix;
#[doc(hidden)] pub mod templating;
#[cfg(any(feature = "tracing", feature = "logging"))]
#[doc(inline)]
pub use templating::{Node, parse, parse_node, render, render_nodes};
#[doc(hidden)] pub mod util;
#[doc(inline)]
pub use util::{
    HEX_RGB_REGEX,
    SINGLE_BAND_DECIMAL_RGB_REGEX,
    SINGLE_BAND_HEX_RGB_REGEX,
    TRIPLE_RGB_REGEX,
    deserialize_string_to_str,
    serialize_static_str_to_string,
};
#[doc(hidden)] pub mod logging;
#[doc(inline)] pub use logging::{setup_logging, setup_tracing};
#[doc(hidden)] pub mod testing;
#[doc(inline)] pub use testing::global_setup;
pub static TERMINAL: LazyLock<TerminalInfo> = LazyLock::new(|| Terminal::info());
