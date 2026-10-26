use std::sync::LazyLock;
use regex::Regex;

pub static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[{]((?<reset>reset)|(?<layer>(fg|color|bg):[#]?([a-fA-F0-9]{6})))[}]").expect("valid regex")
});
