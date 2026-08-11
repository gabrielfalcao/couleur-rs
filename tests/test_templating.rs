// use couleur_rs::{Error, Result};
use couleur_rs::templating::*;
use couleur_rs::{
    Color,
    Contrast,
    // Error,
    Layer,
    RenderableColor,
    Reset,
};
use winnow::{
    Parser,
    error::{ContextError, ErrMode, StrContext},
};
type Result<T> = std::result::Result<T, ContextError>;


/// # TDD TODO:
///
/// ### first set of red -> green -> refactor rounds:
///
///  - [x] parse "{reset}" to `Node::Reset` *DONE:
///  - [x] 2. parse "{color:#F04F78}" to `Node::Color(couleur_rs::Color)`
///  - [x] 3. parse "{color:F04F78}" to `Node::Color(couleur_rs::Color)`
///  - [x] 4. parse "{color:240,79,120}" to `Node::Color(couleur_rs::Color)`
///  - [x] 5. parse "{color:240,  79, 120 , }" to `Node::Color(couleur_rs::Color)`
///
/// ### second set of red -> green -> refactor rounds:
///
///  - [x] 1. parse "hello {reset} world" to `Node::Array(vec![Node::Text("hello "), Node::Reset, Node::Text(" world")])`
///  - [x] 2. parse "{color:#4D9BE6}hello {color:#91DB69}world{reset}" to something like `Node::Array(vec![Node::Color("#4D9BE6".parse::<couleur_rs::Color>()?), Node::Text("hello "), Node::Color("#91DB69".parse::<couleur_rs::Color>()?), Node::Text("world"), Node::Reset])`
///
/// ### third set of red -> green -> refactor rounds:
///
///  - [x] 1. parse "{layer:bg}" to `Node::Layer(couleur_rs::Layer::BG)`
///  - [x] 2. parse "{layer:fg}" to `Node::Layer(couleur_rs::Layer::FG)`
///  - [x] 3. parse "{color:#F9C22B@layer:bg}" to something like `Node::RenderableColor(Node::Layer(couleur_rs::Layer::FG), Node::Color("#F9C22B".parse<couleur_rs::Color>()?))`
///
/// ### fourth set of red -> green -> refactor rounds:
///
///  - [x] 1. parse "{contrast:*VARIANT*}" for each of **variant** of the `couleur_rs::Contrast` enum, that is:
///    - [x] 1.1 "{contrast:none}" should parse to `Node::Contrast(Contrast::None)`
///    - [x] 1.2 "{contrast:read}" should parse to `Node::Contrast(Contrast::Read)`
///    - [x] 1.3 "{contrast:high_bit}" should parse to `Node::Contrast(Contrast::HighBit)`
///    - [x] 1.4 "{contrast:harmonic}" should parse to `Node::Contrast(Contrast::Harmonic)`
///    - [x] 1.5 "{contrast:web}" should parse to `Node::Contrast(Contrast::Web)`
///
///  - [x] 2. parse "{color:#E83B3B%contrast:web}" to something like `Node::RenderableColor(Node::Contrast(Contrast::Web), Node::Color("#E83B3B".parse<couleur_rs::Color>()?))`
///  - [x] 3. parse "{color:#E83B3B@layer:bg%contrast:web}" to something like `Node::RenderableColor(Node::Color("#E83B3B".parse<couleur_rs::Color>()?, Node::Contrast(Contrast::Web), Node::Layer(Layer::BG), ))`
///  - [x] 4. parse "{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World" to something like `Node::Array(vec![Node::Color(Node::Color("#E83B3B".parse<couleur_rs::Color>()?)), Node::Text("Hello"), Node::RenderableColor(Node::Contrast(Contrast::Web), Node::Color("#E83B3B".parse<couleur_rs::Color>()?)), Node::Text(" World")])`
///    - [ ] 4.1 IMPORTANT: take note of this particular test spec and make a reference to it when writing tests for template rendering: "Hello" must be colored with #E83B3B while " World" must be colored with #68BBBB because that's its *"web"* contrast color.
/// 4.0 IMPORTANT: take note of this particular test spec and make a reference to it when writing tests for template rendering: "Hello" must be colored with #E83B3B while " World" must be colored with #68BBBB because that's its *"web"* contrast color.

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
fn test_parse_layer_bg() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{layer:bg}"),
        Ok(("", Node::Layer(Layer::BG)))
    );
    assert_eq!(
        nodes::<ContextError>.parse_peek("{layer:bg}"),
        Ok(("", Node::Array(vec![Node::Layer(Layer::BG)])))
    );

    Ok(())
}
#[test]
fn test_parse_layer_fg() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{layer:fg}"),
        Ok(("", Node::Layer(Layer::FG)))
    );
    assert_eq!(
        nodes::<ContextError>.parse_peek("{layer:fg}"),
        Ok(("", Node::Array(vec![Node::Layer(Layer::FG)])))
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
fn test_parse_contrast_none() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{contrast:none}"),
        Ok(("", Node::Contrast(Contrast::None)))
    );
    assert_eq!(
        nodes::<ContextError>.parse_peek("{contrast:none}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::None)])))
    );

    Ok(())
}
#[test]
fn test_parse_contrast_read() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{contrast:read}"),
        Ok(("", Node::Contrast(Contrast::Read)))
    );
    assert_eq!(
        nodes::<ContextError>.parse_peek("{contrast:read}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::Read)])))
    );

    Ok(())
}
#[test]
fn test_parse_contrast_high_bit() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{contrast:high_bit}"),
        Ok(("", Node::Contrast(Contrast::HighBit)))
    );
    assert_eq!(
        nodes::<ContextError>.parse_peek("{contrast:high_bit}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::HighBit)])))
    );

    Ok(())
}
#[test]
fn test_parse_contrast_harmonic() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{contrast:harmonic}"),
        Ok(("", Node::Contrast(Contrast::Harmonic)))
    );
    assert_eq!(
        nodes::<ContextError>.parse_peek("{contrast:harmonic}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::Harmonic)])))
    );

    Ok(())
}
#[test]
fn test_parse_contrast_web() -> Result<()> {
    assert_eq!(
        parse_node::<ContextError>.parse_peek("{contrast:web}"),
        Ok(("", Node::Contrast(Contrast::Web)))
    );
    assert_eq!(
        nodes::<ContextError>.parse_peek("{contrast:web}"),
        Ok(("", Node::Array(vec![Node::Contrast(Contrast::Web)])))
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
        nodes::<ContextError>.parse_peek("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
        Ok((
            "",
            Node::Array(vec![
                Node::Color("#E83B3B".parse::<couleur_rs::Color>().unwrap()),
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
fn test_render_string() -> Result<()> {
    assert_eq!(
        nodes::<ContextError>.parse_peek("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
        Ok((
            "",
            Node::Array(vec![
                Node::Color("#E83B3B".parse::<couleur_rs::Color>().unwrap()),
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
