// use crate::{Error, Result};
use crate::{Color, Contrast, Error, Layer, Reset, Result};
use std::{collections::HashMap, str, str::FromStr};

use winnow::{
    ascii::{digit1, float, hex_digit1, dec_uint},
    combinator::{alt, cut_err, delimited, preceded, repeat, separated, separated_pair, seq, terminated},
    error::{AddContext, ContextError, ErrMode, ParserError, StrContext},
    prelude::*,
    token::{any, none_of, take, take_while},
};

pub(crate) type Stream<'i> = &'i str;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Reset(Reset),
    Color(Color),
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
pub fn parse_node<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(input: &mut Stream<'i>) -> ModalResult<Node, E> {
    alt((reset.map(Node::Reset), color.map(Node::Color))).parse_next(input)
}

fn reset<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(input: &mut Stream<'i>) -> ModalResult<Reset, E> {
    preceded('{', terminated("reset".value(Reset::default()), '}')).context(StrContext::Expected("reset".into())).parse_next(input)
}
fn color<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(input: &mut Stream<'i>) -> ModalResult<Color, E> {
    preceded(
        '{',
        terminated(
            preceded(
                "color:",
                alt((
                    // separated(0..2, repeat(0..2, digit1), ",").map(|val: String| {
                    //     dbg!(&val);
                    //     val.parse::<Color>().expect("3 decimal numbers")
                    // }),
                    preceded(
                        "#",
                        repeat(0..5, hex_digit1)
                            .fold(|| String::new(), |acc, item| format!("{acc}{item}"))
                            .map(|string| string.parse::<Color>().expect("6 hex digits")),
                    ),
                    repeat(0..5, hex_digit1)
                        .fold(|| String::new(), |acc, item| format!("{acc}{item}"))
                        .map(|string| string.parse::<Color>().expect("6 hex digits")),
                )),
            ),
            '}',
        ),
    )
    .context(StrContext::Expected("rgb color".into()))
    .parse_next(input)
}

fn parse_u8<'i, E: ParserError<Stream<'i>> + std::fmt::Debug + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<u8, E> {
    dec_uint::<Stream<'i>, u8, ErrMode<E>>(input)
    // let val =
    //     repeat(0..2, digit1::<Stream<'i>, E>).fold(|| String::new(), |acc, next| format!("{acc}{next}")).parse_next(input).unwrap();

    // match val.parse::<u8>() {
    //     Ok(val) => Ok(val),
    //     Err(error) => {
    //         // let mut err = ContextError::new();
    //         // err.push(StrContext::Label("digit"));
    //         // err.push(StrContext::Expected("number between 0 and 255".into()));
    //         Err(ErrMode::Cut(ParserError::from_input(input)))
    //     }
    // }
}
fn parse_triple<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<(String, String, String), E> {
    (
        terminated(repeat(0..2, digit1).fold(|| String::new(), |acc, next| format!("{acc}{next}")), ","),
        terminated(repeat(0..2, digit1).fold(|| String::new(), |acc, next| format!("{acc}{next}")), ","),
        alt((
            terminated(repeat(0..2, digit1).fold(|| String::new(), |acc, next| format!("{acc}{next}")), ","),
            repeat(0..2, digit1).fold(|| String::new(), |acc, next| format!("{acc}{next}")),
        )),
    )
        .parse_next(input)
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
    ///  - [ ] 4. parse "{color:240,79,120}" to `Node::Color(crate::Color)`
    ///  - [ ] 5. parse "{color:240,  79, 120 , }" to `Node::Color(crate::Color)`
    ///
    /// ### second set of red -> green -> refactor rounds:
    ///
    ///  - [ ] 1. parse "hello {reset} world" to `Node::Array(vec![Node::Text("hello "), Node::Reset, Node::Text(" world")])`
    ///  - [ ] 2. parse "{color:#4D9BE6}hello {color:#91DB69}world{reset}" to something like `Node::Array(vec![Node::Color("#4D9BE6".parse::<crate::Color>()?), Node::Text("hello "), Node::Color("#91DB69".parse::<crate::Color>()?), Node::Text("world"), Node::Reset])`
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

    use winnow::error::{ContextError, ErrMode};
    type Result<T> = std::result::Result<T, ContextError>;
    use std::str::FromStr;

    #[test]
    fn test_parse_u8() {
        assert_eq!(parse_u8::<ContextError>.parse_peek("127"), Ok(("", 127u8)));
        assert_eq!(parse_u8::<ContextError>.parse_peek("255"), Ok(("", 255u8)));
        assert_eq!(parse_u8::<ContextError>.parse_peek("300"), Err(ErrMode::Backtrack(ContextError::new())));
    }

    #[test]
    fn test_parse_triple_to_string() {
        assert_eq!(parse_triple::<ContextError>.parse_peek("127,255,71,"), Ok(("", ("127".to_string(), "255".to_string(), "71".to_string()))));
        assert_eq!(parse_triple::<ContextError>.parse_peek("127,255,63"), Ok(("", ("127".to_string(), "255".to_string(), "63".to_string()))));
    }

    // #[test]
    // fn test_parse_triple() {
    //     assert_eq!(parse_triple::<ContextError>.parse_peek("127,255,71,"), Ok(("", (127u8, 255u8, 71u8))));
    //     assert_eq!(parse_triple::<ContextError>.parse_peek("127,255,63"), Ok(("", (127u8, 255u8, 63u8))));
    // }

    #[test]
    fn test_parse_reset() -> Result<()> {
        assert_eq!(reset::<ContextError>.parse_peek("{reset}"), Ok(("", Reset::default())));
        assert_eq!(parse_node::<ContextError>.parse_peek("{reset}"), Ok(("", Node::Reset(Reset::default()))));
        Ok(())
    }

    #[test]
    fn test_parse_color_rgb_hex_prefixed_hash() -> Result<()> {
        assert_eq!(color::<ContextError>.parse_peek("{color:#F04F78}"), Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color"))));
        assert_eq!(
            parse_node::<ContextError>.parse_peek("{color:#F04F78}"),
            Ok(("", Node::Color("#F04F78".parse::<Color>().expect("parse rgb color"))))
        );
        Ok(())
    }
    #[test]
    fn test_parse_color_rgb_hex() -> Result<()> {
        assert_eq!(color::<ContextError>.parse_peek("{color:F04F78}"), Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color"))));
        assert_eq!(
            parse_node::<ContextError>.parse_peek("{color:F04F78}"),
            Ok(("", Node::Color("#F04F78".parse::<Color>().expect("parse rgb color"))))
        );
        Ok(())
    }
    // #[test]
    // fn test_parse_color_rgb_u8_triple() -> Result<()> {
    //     assert_eq!(color::<ContextError>.parse_peek("{color:240,79,120}"), Ok(("", "#F04F78".parse::<Color>().expect("parse rgb color"))));
    //     assert_eq!(
    //         parse_node::<ContextError>.parse_peek("{color:240,79,120}"),
    //         Ok(("", Node::Color("#F04F78".parse::<Color>().expect("parse rgb color"))))
    //     );
    //     Ok(())
    // }
}
