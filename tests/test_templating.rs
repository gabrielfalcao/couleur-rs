// use crate::{Error, Result};
pub(self) use crate::{
    AnsiRenderable,
    Color,
    Contrast,
    Error,
    Layer,
    RenderableColor,
    Reset,
    Result,
    Value,
    setup_logging,
};
use couleur::*;

use crate::{setup_logging, setup_tracing};
use winnow::error::{ContextError as Error, ErrMode};
type Result<T> = std::result::Result<T, ContextError>;
use std::{str::FromStr, sync::Once};
use winnow::error::StrContextValue::StringLiteral;

#[test]
fn test_parse_u8() {
    assert_eq!(parse_u8::<Error>.parse_peek("127"), Ok(("", 127u8)));
    // assert_eq!(parse_u8::<Error>.parse_peek("255"), Ok(("", 255u8)));
    // let mut context = ContextError::new();
    context.push(StrContext::Expected("unsigned number between 0 and 255".into()));
    assert_eq!(parse_u8::<Error>.parse_peek("300"), Err(ErrMode::Backtrack(context)));
}

#[test]
fn test_parse_triple_trailing_comma() {
    assert_eq!(parse_triple::<Error>.parse_peek("127,255,71,"), Ok(("", (127u8, 255u8, 71u8))));
}
#[test]
fn test_parse_triple() {
    assert_eq!(parse_triple::<Error>.parse_peek("127,255,63"), Ok(("", (127u8, 255u8, 63u8))));
}

#[test]
fn test_parse_reset() -> Result<()> {
    assert_eq!(reset::<Error>.parse_peek("{reset}"), Ok(("", Reset::default())));
    assert_eq!(parse_node::<Error>.parse_peek("{reset}"), Ok(("", Node::Reset(Reset::default()))));
    Ok(())
}

#[test]
fn test_parse_color_rgb_hex_prefixed_hash() -> Result<()> {
    assert_eq!(
        color::<Error>.parse_peek("{color:#F04F78}"),
        Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color")))
    );
    assert_eq!(
        parse_node::<Error>.parse_peek("{color:#F04F78}"),
        Ok(("", Node::Color("#F04F78".parse::<Color>().expect("parse rgb color"))))
    );
    Ok(())
}
#[test]
fn test_parse_color_rgb_hex() -> Result<()> {
    assert_eq!(
        color::<Error>.parse_peek("{color:F04F78}"),
        Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color")))
    );
    assert_eq!(
        parse_node::<Error>.parse_peek("{color:F04F78}"),
        Ok(("", Node::Color("#F04F78".parse::<Color>().expect("parse rgb color"))))
    );
    Ok(())
}
#[test]
fn test_parse_color_rgb_u8_triple() -> Result<()> {
    assert_eq!(
        color::<Error>.parse_peek("{color:240,79,120,}"),
        Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color")))
    );
    assert_eq!(
        parse_node::<Error>.parse_peek("{color:240,79,120}"),
        Ok(("", Node::Color("#F04F78".parse::<Color>().expect("parse rgb color"))))
    );
    Ok(())
}
#[test]
fn test_parse_color_rgb_u8_triple_with_extra_spaces_and_trailing_comma() -> Result<()> {
    assert_eq!(
        color::<Error>.parse_peek("{color:240,  79, 120 , }"),
        Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color")))
    );
    Ok(())
}
#[test]
fn test_parse_text_single_node() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("hello world"),
        Ok(("", Node::Text("hello world".to_string())))
    );
    Ok(())
}
#[test]
fn test_parse_text_node_array() -> Result<()> {
    assert_eq!(
        nodes::<Error>.parse_peek("hello {reset} world"),
        Ok((
            "",
            Node::Array(vec![
                Node::Text("hello ".to_string()),
                Node::Reset(Reset::default()),
                Node::Text(" world".to_string())
            ])
        ))
    );

    Ok(())
}

#[test]
fn test_parse_node_array_color_text_and_reset() -> Result<()> {
    assert_eq!(
        nodes::<Error>.parse_peek("{color:#4D9BE6}hello {color:#91DB69}world{reset}"),
        Ok((
            "",
            Node::Array(vec![
                Node::Color("#4D9BE6".parse::<crate::Color>().unwrap()),
                Node::Text("hello ".to_string()),
                Node::Color("#91DB69".parse::<crate::Color>().unwrap()),
                Node::Text("world".to_string()),
                Node::Reset(Reset::default())
            ])
        ))
    );

    Ok(())
}
#[test]
fn test_parse_layer_bg() -> Result<()> {
    assert_eq!(parse_node::<Error>.parse_peek("{layer:bg}"), Ok(("", Node::Layer(Layer::BG))));
    assert_eq!(
        nodes::<Error>.parse_peek("{layer:bg}"),
        Ok(("", Node::Array(vec![Node::Layer(Layer::BG)])))
    );

    Ok(())
}
#[test]
fn test_parse_layer_fg() -> Result<()> {
    assert_eq!(parse_node::<Error>.parse_peek("{layer:fg}"), Ok(("", Node::Layer(Layer::FG))));
    assert_eq!(
        nodes::<Error>.parse_peek("{layer:fg}"),
        Ok(("", Node::Array(vec![Node::Layer(Layer::FG)])))
    );

    Ok(())
}
#[test]
fn test_parse_renderable_color_with_layer() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{color:#F9C22B@layer:bg}"),
        Ok((
            "",
            Node::RenderableColor(
                RenderableColor::new("#F9C22B".parse::<crate::Color>().unwrap())
                    .with_layer(Layer::BG)
            )
        ))
    );

    Ok(())
}

#[test]
fn test_parse_contrast_none() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{contrast:none}"),
        Ok(("", Node::Contrast(Contrast::None)))
    );
    assert_eq!(
        nodes::<Error>.parse_peek("{contrast:none}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::None)])))
    );

    Ok(())
}
#[test]
fn test_parse_contrast_read() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{contrast:read}"),
        Ok(("", Node::Contrast(Contrast::Read)))
    );
    assert_eq!(
        nodes::<Error>.parse_peek("{contrast:read}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::Read)])))
    );

    Ok(())
}
#[test]
fn test_parse_contrast_high_bit() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{contrast:high_bit}"),
        Ok(("", Node::Contrast(Contrast::HighBit)))
    );
    assert_eq!(
        nodes::<Error>.parse_peek("{contrast:high_bit}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::HighBit)])))
    );

    Ok(())
}
#[test]
fn test_parse_contrast_harmonic() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{contrast:harmonic}"),
        Ok(("", Node::Contrast(Contrast::Harmonic)))
    );
    assert_eq!(
        nodes::<Error>.parse_peek("{contrast:harmonic}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::Harmonic)])))
    );

    Ok(())
}
#[test]
fn test_parse_contrast_web() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{contrast:web}"),
        Ok(("", Node::Contrast(Contrast::Web)))
    );
    assert_eq!(
        nodes::<Error>.parse_peek("{contrast:web}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::Web)])))
    );

    Ok(())
}
#[test]
fn test_parse_renderable_color_with_contrast() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{color:#E83B3B%contrast:web}"),
        Ok((
            "",
            Node::RenderableColor(
                RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
                    .with_contrast(Contrast::Web)
            )
        ))
    );

    Ok(())
}
#[test]
fn test_parse_renderable_color_with_layer_and_contrast() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{color:#E83B3B@layer:bg%contrast:web}"),
        Ok((
            "",
            Node::RenderableColor(
                RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
                    .with_layer(Layer::BG)
                    .with_contrast(Contrast::Web)
            )
        ))
    );

    Ok(())
}
#[test]
fn test_parse_renderable_color_with_layer_contrast_and_text() -> Result<()> {
    assert_eq!(
        nodes::<Error>.parse_peek("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
        Ok((
            "",
            Node::Array(vec![
                Node::Color("#E83B3B".parse::<crate::Color>().unwrap()),
                Node::Text("Hello".to_string()),
                Node::RenderableColor(
                    RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
                        .with_contrast(Contrast::Web)
                ),
                Node::Text(" World".to_string())
            ])
        ))
    );
    assert_eq!(
        parse::<&str, Error>("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
        Ok(Node::Array(vec![
            Node::Color("#E83B3B".parse::<crate::Color>().unwrap()),
            Node::Text("Hello".to_string()),
            Node::RenderableColor(
                RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
                    .with_contrast(Contrast::Web)
            ),
            Node::Text(" World".to_string())
        ]))
    );

    Ok(())
}
#[test]
fn test_render_string() -> Result<()> {
    assert_eq!(
        nodes::<Error>.parse_peek("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
        Ok((
            "",
            Node::Array(vec![
                Node::Color("#E83B3B".parse::<crate::Color>().unwrap()),
                Node::Text("Hello".to_string()),
                Node::RenderableColor(
                    RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
                        .with_contrast(Contrast::Web)
                ),
                Node::Text(" World".to_string())
            ])
        ))
    );
    assert_eq!(
        parse::<&str, Error>("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
        Ok(Node::Array(vec![
            Node::Color("#E83B3B".parse::<crate::Color>().unwrap()),
            Node::Text("Hello".to_string()),
            Node::RenderableColor(
                RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
                    .with_contrast(Contrast::Web)
            ),
            Node::Text(" World".to_string())
        ]))
    );

    Ok(())
}
