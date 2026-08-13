// use couleur_rs::{Error, Result};
use couleur_rs::{
    Color,
    Contrast,
    // Error,
    Layer,
    RenderableColor,
    Reset,
};
use couleur_rs::{global_setup, templating::*};
use pretty_assertions::assert_eq;
use winnow::{
    Parser,
    error::{ContextError, ErrMode, StrContext},
};
type Result<T> = std::result::Result<T, ContextError>;

#[test]
fn test_parse_u8() {
    assert_eq!(parse_u8::<ContextError>.parse_peek("127"), Ok(("", 127u8)));
    assert_eq!(parse_u8::<ContextError>.parse_peek("255"), Ok(("", 255u8)));
    let mut context = ContextError::new();
    context.push(StrContext::Expected("unsigned number between 0 and 255".into()));
    assert_eq!(parse_u8::<ContextError>.parse_peek("300"), Err(ErrMode::Backtrack(context)));
}

#[test]
fn test_parse_triple_trailing_comma() {
    assert_eq!(
        parse_triple::<ContextError>.parse_peek("127,255,71,"),
        Ok(("", (127u8, 255u8, 71u8)))
    );
}
#[test]
fn test_parse_triple() {
    assert_eq!(
        parse_triple::<ContextError>.parse_peek("127,255,63"),
        Ok(("", (127u8, 255u8, 63u8)))
    );
}

#[test]
fn test_parse_reset() -> Result<()> {
    assert_eq!(reset::<ContextError>.parse_peek("{reset}"), Ok(("", Reset::default())));
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{reset}"),
        Ok(("", Node::Reset(Reset::default())))
    );
    Ok(())
}

#[test]
fn test_parse_color_rgb_hex_prefixed_hash() -> Result<()> {
    assert_eq!(
        color::<ContextError>.parse_peek("{color:#F04F78}"),
        Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color")))
    );
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{color:#F04F78}"),
        Ok(("", Node::Color("#F04F78".parse::<Color>().expect("parse rgb color"))))
    );
    Ok(())
}
#[test]
fn test_parse_color_rgb_hex() -> Result<()> {
    assert_eq!(
        color::<ContextError>.parse_peek("{color:F04F78}"),
        Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color")))
    );
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{color:F04F78}"),
        Ok(("", Node::Color("#F04F78".parse::<Color>().expect("parse rgb color"))))
    );
    Ok(())
}
#[test]
fn test_parse_color_rgb_u8_triple() -> Result<()> {
    assert_eq!(
        color::<ContextError>.parse_peek("{color:240,79,120,}"),
        Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color")))
    );
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{color:240,79,120}"),
        Ok(("", Node::Color("#F04F78".parse::<Color>().expect("parse rgb color"))))
    );
    Ok(())
}
#[test]
fn test_parse_color_rgb_u8_triple_with_extra_spaces_and_trailing_comma() -> Result<()> {
    assert_eq!(
        color::<ContextError>.parse_peek("{color:240,  79, 120 , }"),
        Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color")))
    );
    Ok(())
}
#[test]
fn test_parse_text_single_node() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("hello world"),
        Ok(("", Node::Text("hello world".to_string())))
    );
    Ok(())
}
#[test]
fn test_parse_text_node_array() -> Result<()> {
    assert_eq!(
        nodes::<ContextError>.parse_peek("hello {reset} world"),
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
        nodes::<ContextError>.parse_peek("{color:#4D9BE6}hello {color:#91DB69}world{reset}"),
        Ok((
            "",
            Node::Array(vec![
                Node::Color("#4D9BE6".parse::<couleur_rs::Color>().unwrap()),
                Node::Text("hello ".to_string()),
                Node::Color("#91DB69".parse::<couleur_rs::Color>().unwrap()),
                Node::Text("world".to_string()),
                Node::Reset(Reset::default())
            ])
        ))
    );

    Ok(())
}
#[test]
fn test_parse_renderable_color_with_layer() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{color:#F9C22B@layer:bg}"),
        Ok((
            "",
            Node::RenderableColor(
                RenderableColor::new("#F9C22B".parse::<couleur_rs::Color>().unwrap())
                    .with_layer(Layer::BG)
            )
        ))
    );

    Ok(())
}

#[test]
fn test_parse_renderable_color_with_contrast() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{color:#E83B3B%contrast:web}"),
        Ok((
            "",
            Node::RenderableColor(
                RenderableColor::new("#E83B3B".parse::<couleur_rs::Color>().unwrap())
                    .with_contrast(Contrast::Web)
            )
        ))
    );

    Ok(())
}
#[test]
fn test_parse_renderable_color_with_layer_and_contrast() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{color:#E83B3B@layer:bg%contrast:web}"),
        Ok((
            "",
            Node::RenderableColor(
                RenderableColor::new("#E83B3B".parse::<couleur_rs::Color>().unwrap())
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
        nodes::<ContextError>
            .parse_peek("{color:#C32454}Hello{color:#C32454%contrast:web} World{reset}"),
        Ok((
            "",
            Node::Array(vec![
                Node::Color("#C32454".parse::<couleur_rs::Color>().unwrap()),
                Node::Text("Hello".to_string()),
                Node::RenderableColor(
                    RenderableColor::new("#C32454".parse::<couleur_rs::Color>().unwrap())
                        .with_contrast(Contrast::Web)
                ),
                Node::Text(" World".to_string()),
                Node::Reset(Default::default())
            ])
        ))
    );
    assert_eq!(
        parse::<&str, ContextError>(
            "{color:#C32454}Hello{color:#C32454%contrast:web} World{reset}"
        ),
        Ok(Node::Array(vec![
            Node::Color("#C32454".parse::<couleur_rs::Color>().unwrap()),
            Node::Text("Hello".to_string()),
            Node::RenderableColor(
                RenderableColor::new("#C32454".parse::<couleur_rs::Color>().unwrap())
                    .with_contrast(Contrast::Web)
            ),
            Node::Text(" World".to_string()),
            Node::Reset(Default::default())
        ]))
    );

    Ok(())
}

#[test]
fn test_render_string() -> Result<()> {
    assert_eq!(
        nodes::<ContextError>
            .parse_peek("{color:#E83B3B@layer:bg}Hello{color:#E83B3B%contrast:web} World"),
        Ok((
            "",
            Node::Array(vec![
                Node::RenderableColor(
                    RenderableColor::new("#E83B3B".parse::<couleur_rs::Color>().unwrap())
                        .with_layer(Layer::BG)
                ),
                Node::Text("Hello".to_string()),
                Node::RenderableColor(
                    RenderableColor::new("#E83B3B".parse::<couleur_rs::Color>().unwrap())
                        .with_contrast(Contrast::Web)
                ),
                Node::Text(" World".to_string())
            ])
        ))
    );
    assert_eq!(
        parse::<&str, ContextError>("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
        Ok(Node::Array(vec![
            Node::Color("#E83B3B".parse::<couleur_rs::Color>().unwrap()),
            Node::Text("Hello".to_string()),
            Node::RenderableColor(
                RenderableColor::new("#E83B3B".parse::<couleur_rs::Color>().unwrap())
                    .with_contrast(Contrast::Web)
            ),
            Node::Text(" World".to_string())
        ]))
    );

    Ok(())
}

#[test]
fn test_parse_contrast_of_color() -> Result<()> {
    global_setup();
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
