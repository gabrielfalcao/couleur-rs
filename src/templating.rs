// use crate::{Error, Result};
use crate::{Color, Contrast, Error, Layer, Reset, Result, Value, setup_logging};
use std::str::FromStr;
use tracing::{Level, event, instrument, span};

use winnow::{
    ascii::{dec_uint, digit1, float, hex_digit1},
    combinator::{
        alt,
        cut_err,
        delimited,
        eof,
        iterator,
        preceded,
        repeat,
        separated,
        separated_pair,
        seq,
        terminated,
    },
    error::{AddContext, ContextError, ErrMode, ParserError, StrContext},
    prelude::*,
    token::{any, none_of, rest, take, take_while},
};

pub(crate) type Stream<'i> = &'i str;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Reset(Reset),
    Color(Color),
    Text(String),
    Layer(Layer),
    Array(Vec<Node>),
}
impl From<Reset> for Node {
    fn from(reset: Reset) -> Node {
        Node::Reset(reset)
    }
}
impl From<Color> for Node {
    fn from(color: Color) -> Node {
        Node::Color(color)
    }
}

#[instrument]
pub fn parse_node<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Node, E> {
    span!(Level::TRACE, "input", input);
    if input.is_empty() {
        return Err(ErrMode::Cut(ParserError::from_input(input)));
    }
    // log::debug!("parse_node called with input: {input:#?}", &input);
    alt((
        reset::<E>.map(Node::Reset), // Reset
        color::<E>.map(Node::Color), // Color
        text::<E>.map(Node::Text),   // Text
        nodes::<E>,                  // Array
    ))
    .parse_next(input)
}

#[instrument]
fn reset<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Reset, E> {
    span!(Level::TRACE, "input", input);
    preceded('{', terminated("reset".value(Reset::default()), '}'))
        .context(StrContext::Expected("reset".into()))
        .parse_next(input)
}
#[instrument]
fn color<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    span!(Level::TRACE, "input", input);
    preceded(
        '{',
        terminated(
            preceded(
                "color:",
                alt((parse_rgb_triple, preceded("#", parse_rgb_hex), parse_rgb_hex)),
            ),
            '}',
        ),
    )
    .context(StrContext::Expected("rgb color".into()))
    .parse_next(input)
}

#[instrument]
fn text<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<String, E> {
    span!(Level::TRACE, "input", input);
    alt((take_while(0.., |c: char| c != '{').context(StrContext::Expected("text".into())),))
        .parse_next(input)
        .map(|s| s.to_string())
}
#[instrument]
fn parse_u8<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<u8, E> {
    span!(Level::TRACE, "input", input);
    dec_uint::<Stream<'i>, u8, ErrMode<E>>
        .context(StrContext::Expected("unsigned number between 0 and 255".into()))
        .parse_next(input)
}
#[instrument]
fn parse_rgb_hex<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    span!(Level::TRACE, "input", input);
    repeat(0..5, hex_digit1)
        .fold(|| String::new(), |acc, item| format!("{acc}{item}"))
        .context(StrContext::Expected("6 hex digits".into()))
        .map(|string| string.parse::<Color>().expect("6 hex digits"))
        .parse_next(input)
}
#[instrument]
fn parse_rgb_triple<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    span!(Level::TRACE, "input", input);
    parse_triple
        .map(|(red, green, blue)| Color::from_triple(red.into(), green.into(), blue.into()))
        .parse_next(input)
}
#[instrument]
fn parse_triple<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<(u8, u8, u8), E> {
    span!(Level::TRACE, "input", input);
    (
        terminated(parse_u8, (ws, ',', ws)),
        terminated(parse_u8, (ws, ',', ws)),
        alt((terminated(parse_u8, (ws, ',', ws)), parse_u8)),
    )
        .context(StrContext::Expected("three comma-separated unsigned numbers".into()))
        .parse_next(input)
}
#[instrument]
fn ws<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<&'i str, E> {
    span!(Level::TRACE, "input", input);
    take_while(0.., &[' ', '\t', '\r', '\n'])
        .context(StrContext::Expected("white space".into()))
        .parse_next(input)
}
#[instrument]
fn nodes<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    mut input: &mut Stream<'i>,
) -> ModalResult<Node, E> {
    span!(Level::TRACE, "input", input);
    let mut winnow_it = iterator(input, parse_node::<E>);
    let res = winnow_it.map(|node| node).collect::<Vec<Node>>();

    winnow_it.finish();
    Ok(Node::Array(res))
}

#[cfg(test)]
mod tests {
    /// # TDD TODO:
    ///
    /// ### first set of red -> green -> refactor rounds:
    ///
    ///  - [x] parse "{reset}" to `Node::Reset` *DONE:
    ///  - [x] 2. parse "{color:#F04F78}" to `Node::Color(crate::Color)`
    ///  - [x] 3. parse "{color:F04F78}" to `Node::Color(crate::Color)`
    ///  - [x] 4. parse "{color:240,79,120}" to `Node::Color(crate::Color)`
    ///  - [x] 5. parse "{color:240,  79, 120 , }" to `Node::Color(crate::Color)`
    ///
    /// ### second set of red -> green -> refactor rounds:
    ///
    ///  - [x] 1. parse "hello {reset} world" to `Node::Array(vec![Node::Text("hello "), Node::Reset, Node::Text(" world")])`
    ///  - [x] 2. parse "{color:#4D9BE6}hello {color:#91DB69}world{reset}" to something like `Node::Array(vec![Node::Color("#4D9BE6".parse::<crate::Color>()?), Node::Text("hello "), Node::Color("#91DB69".parse::<crate::Color>()?), Node::Text("world"), Node::Reset])`
    ///
    /// ### third set of red -> green -> refactor rounds:
    ///
    ///  - [ ] 1. parse "{layer:bg}" to `Node::Layer(crate::Layer::BG)`
    ///  - [ ] 2. parse "{layer:fg}" to `Node::Layer(crate::Layer::FG)`
    ///  - [ ] 3. parse "{color:#F9C22B@layer:bg}" to `Node::AnsiLayered(Node::Layer(crate::Layer::FG), Node::Color("#F9C22B".parse<crate::Color>()?))`
    ///
    /// ### forth set of red -> green -> refactor rounds:
    ///
    ///  - [ ] 1. parse "{contrast:*VARIANT*}" for each of **variant** of the `crate::Contrast` enum, that is:
    ///    - [ ] 1.1 "{contrast:none}" should parse to `Node::Contrast(Contrast::None)`
    ///    - [ ] 1.2 "{contrast:read}" should parse to `Node::Contrast(Contrast::Read)`
    ///    - [ ] 1.3 "{contrast:high_bit}" should parse to `Node::Contrast(Contrast::HighBit)`
    ///    - [ ] 1.4 "{contrast:harmonic}" should parse to `Node::Contrast(Contrast::Harmonic)`
    ///    - [ ] 1.5 "{contrast:web}" should parse to `Node::Contrast(Contrast::Web)`
    ///
    ///  - [ ] 2. parse "{color:#E83B3B%contrast:web}" to `Node::ContrastedColor(Node::Contrast(Contrast::Web), Node::Color("#E83B3B".parse<crate::Color>()?))`
    ///  - [ ] 3. parse "{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World" to `Node::Array(vec![Node::Color(Node::Color("#E83B3B".parse<crate::Color>()?)), Node::Text("Hello"), Node::ContrastedColor(Node::Contrast(Contrast::Web), Node::Color("#E83B3B".parse<crate::Color>()?)), Node::Text(" World")])`
    ///    - [ ] 3.1 IMPORTANT: take note of this particular test spec and make a reference to it when writing tests for template rendering: "Hello" must be colored with #E83B3B while " World" must be colored with #68BBBB because that's its *"web"* contrast color.
    use super::*;
    use crate::{setup_logging, setup_tracing};
    use winnow::error::{ContextError as Error, ErrMode};
    type Result<T> = std::result::Result<T, ContextError>;
    use std::{str::FromStr, sync::Once};
    use winnow::error::StrContextValue::StringLiteral;

    static INIT: Once = Once::new();

    fn global_setup() {
        INIT.call_once(|| {
            setup_logging().expect("setup logging");
            // setup_tracing().expect("setup tracing");
        });
    }

    #[test]
    fn test_parse_u8() {
        assert_eq!(parse_u8::<Error>.parse_peek("127"), Ok(("", 127u8)));
        assert_eq!(parse_u8::<Error>.parse_peek("255"), Ok(("", 255u8)));
        let mut context = ContextError::new();
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
        assert_eq!(
            parse_node::<Error>.parse_peek("{reset}"),
            Ok(("", Node::Reset(Reset::default())))
        );
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
}
