use {
    serde::{Deserialize, Deserializer, Serializer},
    std::sync::LazyLock,
};

use regex::Regex;

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

#[doc(hidden)]
pub fn serialize_static_str_to_string<S>(val: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&val.to_string())
}

#[doc(hidden)]
pub fn deserialize_string_to_str<'de, D>(deserializer: D) -> Result<&'de str, D::Error>
where
    D: Deserializer<'de>,
{
    let mut raw_string = String::deserialize(deserializer)?;
    let leaked = raw_string.leak();
    Ok(leaked)
}
