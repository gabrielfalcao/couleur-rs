pub(crate) mod color;
pub use color::{Color, RGBParseError};

pub(crate) mod triples;
pub use triples::{RgbTriple, U8Triple};

pub(crate) mod constants;
pub use constants::{BLACK, WHITE};
