pub(crate) mod rgb_value;
pub use rgb_value::RGBValue;

pub(crate) mod util;
pub use util::{
    HEX_RGB_REGEX, SINGLE_BAND_DECIMAL_RGB_REGEX, SINGLE_BAND_HEX_RGB_REGEX, TRIPLE_RGB_REGEX,
};

pub(crate) mod rgb_color;
pub use rgb_color::{BLACK, RGB_COLOR_REGEX, RGBColor, RGBParseError, WHITE};

pub(crate) mod triples;
pub use triples::{RgbTriple, U8Triple};

pub(crate) mod cmp;
pub use cmp::{max_rgb, min_rgb};
