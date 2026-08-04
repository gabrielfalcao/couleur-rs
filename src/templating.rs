// use crate::{Error, Result};
use crate::{Color, Contrast, Error, Layer, Reset, Result};
use std::collections::HashMap;
use std::str;

use winnow::prelude::*;
use winnow::{
    ascii::float,
    combinator::alt,
    combinator::cut_err,
    combinator::{delimited, preceded, separated_pair, terminated},
    combinator::{repeat, separated},
    error::{AddContext, ParserError, StrContext},
    token::{any, none_of, take, take_while},
};

pub(crate) type Stream<'i> = &'i str;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Reset(Reset),
}
impl From<Reset> for Node {
    fn from(reset: Reset ) -> Node {
        Node::Reset(reset)
    }
}
pub fn parse_node<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Node, E> {
    // `alt` combines the each value parser. It returns the result of the first
    // successful parser, or an error
    alt((
        reset.map(Node::Reset),
    ))
    .parse_next(input)
}


fn reset<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Reset, E> {
    preceded(
        '{',
        cut_err(terminated("reset".value(Reset::default()), '}')),
    )
    .context(StrContext::Expected("reset".into()))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    /// # TDD TODO:
    ///
    /// ### first set of red -> green -> refactor rounds:
    ///
    ///  - [x] parse "{reset}" to `Node::Reset` *DONE:
    ///  - [ ] 2. parse "{color:#F04F78}" to `Node::Color(crate::Color)`
    ///  - [ ] 3. parse "{color:F04F78}" to `Node::Color(crate::Color)`
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
    use k9::assert_equal;
    use winnow::error::ContextError as Error;
    type Result<T> = std::result::Result<T, Error>;

    #[test]
    fn test_parse_reset() -> Result<()> {
        assert_equal!(reset::<Error>.parse_peek("{reset}"), Ok(("", Reset::default())));
        assert_equal!(parse_node::<Error>.parse_peek("{reset}"), Ok(("", Node::Reset(Reset::default()))));
        Ok(())
    }
}
