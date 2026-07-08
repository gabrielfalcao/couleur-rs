pub(crate) mod rgb_value;
pub use rgb_value::RGBValue;

pub(crate) mod util;
pub use util::{
    HEX_RGB_REGEX, SINGLE_BAND_DECIMAL_RGB_REGEX, SINGLE_BAND_HEX_RGB_REGEX, TRIPLE_RGB_REGEX,
};
