#![allow(unused)]
// use couleur_rs::{Error, Result};
use couleur_rs::{
    Color,
    Contrast,
    // Error,
    Layer,
    RenderableColor,
    Reset,
    parse,
};
use couleur_rs::{global_setup, templating::*};
use pretty_assertions::assert_eq;
use winnow::{
    Parser,
    error::{ContextError, ErrMode, StrContext},
};
type Result<T> = std::result::Result<T, ContextError>;

#[test]
fn test_parse_reset() -> Result<()> {
    assert_eq!(parse("{reset}"), Ok(format!("0m")));
    Ok(())
}

#[test]
fn test_parse_color_rgb_hex_prefixed_hash() -> Result<()> {
    assert_eq!(parse("{color:#F04F78}"), Ok("\x1b[1;38;2;240;79;120m".to_string()));
    Ok(())
}
#[test]
fn test_parse_color_surrounded_by_reset() -> Result<()> {
    assert_eq!(parse("{reser}{color:#F04F78}hello{reset}"), Ok("\x1b[1;38;2;240;79;120m".to_string()));
    Ok(())
}
