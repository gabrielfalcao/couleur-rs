#[doc(hidden)] pub mod color;
#[doc(inline)] pub use color::{Color, RGBParseError};

#[doc(hidden)] pub mod triples;
#[doc(inline)] pub use triples::{RgbTriple, U8Triple};

#[doc(hidden)] pub mod constants;
#[doc(inline)] pub use constants::{BLACK, WHITE};

#[doc(hidden)] pub mod ansi256;
#[doc(inline)] pub use ansi256::convert_ansi256_to_rgb_triple;
