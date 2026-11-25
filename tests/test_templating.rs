#![allow(unused)]
// use couleur_rs::{Error, Result};
use couleur_rs::{
    Color,
    Contrast,
    // Error,
    Layer,
    RenderableColor,
    Reset,
    render,
};
use couleur_rs::{global_setup, templating::*};
use pretty_assertions::assert_eq;
use winnow::{
    Parser,
    error::{ContextError, ErrMode, StrContext},
};
type Result<T> = std::result::Result<T, ContextError>;

#[test]
fn test_render_reset() -> Result<()> {
    assert_eq!(render("{reset}"), Ok(format!("\x1b[0m")));
    Ok(())
}

#[test]
fn test_render_color_rgb_hex_prefixed_hash() -> Result<()> {
    assert_eq!(render("{color:#F04F78}"), Ok("\x1b[1;38;2;240;79;120m".to_string()));
    Ok(())
}
#[test]
fn test_render_color_surrounded_by_reset() -> Result<()> {
    assert_eq!(render("{reser}{color:#F04F78}hello{reset}"), Ok("\x1b[1;38;2;240;79;120mhello\x1b[0m".to_string()));
    Ok(())
}

#[test]
fn test_parse_contrast_of_color() -> Result<()> {
    assert_eq!(
        nodes::<ContextError>.parse_peek("{contrasted_color:#676633:web}Hello World{reset}"),
        Ok((
            "",
            Node::Array(vec![
                Node::Color("#E7E6B3".parse::<couleur_rs::Color>().unwrap()),
                Node::Text("Hello World".to_string()),
                Node::Reset(Default::default())
            ])
        ))
    );

    Ok(())
}
