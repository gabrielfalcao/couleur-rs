// use crate::{Error, Result};
use winnow::{
    ModalResult,
    Parser,
    Result,
    ascii::{alpha1, alphanumeric1 as alphanumeric, float, take_escaped},
    combinator::{alt, cut_err, delimited, preceded, repeat, repeat_till, separated, separated_pair, terminated},
    error::{ContextError, ParserError, StrContext},
    prelude::*,
    stream::{Offset, Stream},
    token::{literal, none_of, one_of, rest, take_until, take_while},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    Sequence(Vec<Node>),
    String(String),
    Unhandled(String),
    Reset,
    None,
}

fn parse_str<'a, E: ParserError<&'a str>>(input: &mut &'a str) -> ModalResult<&'a str, E> {
    alpha1.parse_next(input)
}

fn parse_markup<'a>(input: &mut &'a str) -> ModalResult<Node> {
    // // preceded('{', cut_err(terminated(alpha1, '}'))).context(StrContext::Label("markup")).parse_next(input)
    // delimited("{", alpha1, "}").parse_next(&mut input)
    let start = input.checkpoint();

    // alt((
    //     alph
    //         preceded('{', cut_err(terminated(parse_str, '}')))
    //             .context(StrContext::Label("trying to parse markup wrapped by curly braces"))
    //             ))
    //     .parse_next(input)
    if let Ok(markup) = preceded('{', cut_err(terminated(parse_str::<crate::ParseError>, '}')))
        .context(StrContext::Label("trying to parse markup wrapped by curly braces"))
        .parse_next(input)
        .map(|value| value.to_string())
    {
        return Ok(match markup.as_str() {
            "reset" => Node::Reset,
            other => Node::Unhandled(other.to_string()),
        });
    }
    input.reset(&start);

    if let Ok(anything_else) = rest::<&str, ContextError>.parse_next(input) {
        return Ok(Node::String(anything_else.to_string()));
    }

    Err(ParserError::from_input(input))
}

// fn parse_nodes<'a>(input: &mut &'a str) -> Result<Node<'a>> {
//     let result = delimited("{", none_of(['}', '{']), "}");.parse_next(input)
//
//     // let actual = input.next_slice(expected.len());
//     // if actual != expected {
//     //     return Err(ParserError::from_input(input));
//     // }
//     // Ok(actual)
//     Ok(Node::None)
// }

#[cfg(test)]
mod tests {
    /// examples:
    ///
    /// "hello {reset} world"
    /// {color:#FB6B1D,layer:bg}
    /// {color:terminal_foreground,layer:bg}
    /// {color:resurrect-64:cold_green_medium,layer:bg}
    use super::*;
    use k9::assert_equal;

    #[test]
    fn test_parse_hardcoded_reset_keyword_wrapped_in_braces_markup() -> crate::Result<()> {
        let mut input = "{reset}";
        let result = dbg!(parse_markup(&mut input));

        assert_equal!(result, Ok(Node::Reset));

        Ok(())
    }

//
//
}
