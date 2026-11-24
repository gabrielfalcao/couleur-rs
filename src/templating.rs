use std::{fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;

use crate::{AnsiRenderable, Color, Error, Layer, Prefix, Reset, Result, ToAnsiEscSuffix};

#[doc(hidden)] pub mod node;
#[doc(inline)] pub use node::{Node, NodeType, fold_nodes};

pub static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?<markup>[{]((?<reset>reset)|(?<layer>(fg|color|bg):[#]?(?<color>[a-fA-F0-9]{6})))[}]|(?P<text>[^{}]+))+")
        .expect("valid regex")
});

pub fn render<T: Display>(input: T) -> Result<String> {
    let input = input.to_string();
    let mut tokens = Vec::<String>::new();
    for parsed in REGEX.captures_iter(&input) {
        let prefix = Prefix::default().to_string();
        let layer = if let Some(layer) = parsed.name("layer") {
            match layer.as_str() {
                "bg" => Layer::BG,
                _ => Layer::FG,
            }
        } else {
            Layer::default()
        };
        let reset = if let Some(reset) = parsed.name("reset") {
            Reset::default().to_ansi_esc_suffix()
        } else {
            String::new()
        };
        let color = if let Some(color) = parsed.name("color") {
            let color = color.as_str().parse::<Color>()?;
            color.to_ansi(layer)
        } else {
            String::new()
        };
        let text = if let Some(text) = parsed.name("text") {
            text.as_str().to_string()
        } else {
            String::new()
        };
        if color.len() > 0 {
            tokens.push(color.to_string());
            tokens.push(text.to_string());
        }
        if reset.len() > 0 {
            tokens.push(prefix.clone());
            tokens.push(reset.clone());
        }

        // match (layer, color) {
        //     (Some(layer), Some(color)) => {
        //         let layer = Layer::from_str(&layer.as_str())?;
        //         let color = Color::from_str(&color.as_str())?.to_ansi(layer);
        //     }
        //     (Some(layer), None) => {
        //         return Err(Error::RuntimeError(format!(
        //             r"missing RGB color from template syntax (.i.e.: \x7bcolor:#ffcc00\7d)"
        //         )));
        //     }
        //     (None, Some(color)) => {
        //         let color = Color::from_str(&color.as_str())?;
        //         let layer = Layer::default();
        //         tokens.push(layer.to_string());
        //         tokens.push(color.to_string());
        //     }
        //     (None, None) => {
        //         return Err(Error::RuntimeError(format!("could not parse string: {input}")));
        //     }
        // }
    }
    Ok(tokens.join("").replace("[[", "[").to_string())
}
