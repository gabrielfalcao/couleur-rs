use std::{fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;

use crate::{Color, Error, Layer, Reset, Result};

pub static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[{]((?<reset>reset)|(?<layer>(fg|color|bg):[#]?(?<color>[a-fA-F0-9]{6})))[}]")
        .expect("valid regex")
});

pub fn parse<T: Display>(input: T) -> Result<String> {
    let input = input.to_string();
    let mut tokens = Vec::<String>::new();
    for parsed in REGEX.captures_iter(&input) {
        dbg!(&parsed);
        let reset = parsed.name("reset");
        let layer = parsed.name("layer");
        let color = parsed.name("color");
        if let Some(reset) = reset {
            tokens.push(Reset::default().to_string());
        } else {
            match (layer, color) {
                (Some(layer), Some(color)) => {
                    let color = Color::from_str(&color.as_str())?;
                    let layer = Layer::from_str(&layer.as_str())?;
                    tokens.push(layer.to_string());
                    tokens.push(color.to_string());
                }
                (Some(layer), None) => {
                    return Err(Error::RuntimeError(format!(r"missing RGB color from template syntax (.i.e.: \x7bcolor:#ffcc00\7d)")))
                }
                (None, Some(color)) => {
                    let color = Color::from_str(&color.as_str())?;
                    let layer = Layer::default();
                    tokens.push(layer.to_string());
                    tokens.push(color.to_string());
                }
                (None, None) => {
                    return Err(Error::RuntimeError(format!("could not parse string: {input}")))
                }
            }
        }
    }
    Ok(tokens.join(""))
}
