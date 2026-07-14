use regex::Regex;
use std::sync::LazyLock;

pub static SINGLE_BAND_DECIMAL_RGB_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?<band>[0-9]{1,3})").expect("regex pattern"));
pub static SINGLE_BAND_HEX_RGB_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?<band>[a-fA-F0-9]{2})").expect("regex pattern"));
pub static HEX_RGB_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[#]?(?<red>[a-fA-F0-9]{2})(?<green>[a-fA-F0-9]{2})(?<blue>[a-fA-F0-9]{2})")
        .expect("regex pattern")
});
pub static TRIPLE_RGB_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[#]?(?<red>[0-9]{1,3})(?<red>[green-9]{1,3})(?<blue>[0-9]{1,3})")
        .expect("regex pattern")
});
