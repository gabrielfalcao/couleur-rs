use crate::{Color, ParseError};
use winnow::{
    ModalResult,
    Parser,
    Result,
    ascii::{
        alpha1,
        alphanumeric1 as alphanumeric,
        float,
        hex_digit0,
        hex_digit1,
        take_escaped,
    },
    combinator::{
        alt,
        cut_err,
        delimited,
        impls::Context,
        preceded,
        repeat,
        repeat_till,
        separated,
        separated_pair,
        terminated,
    },
    error::{ContextError, ErrMode, ParserError, StrContext},
    prelude::*,
    stream::{AsChar, Offset, Stream},
    token::{literal, none_of, one_of, rest, take_till, take_while},
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    Sequence(Vec<Node>),
    String(String),
    Color(Color),
    Unhandled(String),
    Reset,
    None,
}

fn parse_str<'a, E: ParserError<&'a str>>(
    input: &mut &'a str,
) -> ModalResult<&'a str, E> {
    alt((
        alpha1,
        separated_pair(
            "color",
            ":",
            alt((
                preceded("#", repeat(6, hex_digit1)),
                repeat(6, hex_digit1),
            )),
        ),
    ))
    .parse_next(input)
}
fn parse_markup<'a>(input: &mut &'a str) -> ModalResult<Node> {
    let original_input = input.to_string();
    let mut nodes = Vec::<Node>::new();

    while input.len() > 0 {
        if let Ok(markup) =
            // delimited('{', parse_str::<crate::ParseError>, '}')
            preceded(
                '{',
                cut_err(terminated(parse_str::<crate::ParseError>, '}')),
            )
            .context(StrContext::Label(
                "trying to parse markup wrapped by curly braces",
            ))
            .parse_next(input)
            .map(|value: &str| value.to_string())
        {
            nodes.push(match markup.as_str() {
                "reset" => Node::Reset,
                other => Node::Unhandled(other.to_string()),
            });
        } else if let Ok(anything_else) =
            take_while::<_, &str, ParseError>(0.., |c| c != '{').parse_next(input)
        {
            nodes.push(Node::String(anything_else.to_string()));
        } else {
            return Err(ErrMode::Cut(
                ParseError::new("unexpected case")
                    .with_input(original_input.to_string())
                    .with_context("trying to parse markup")
                    .into(),
            ));
        }
    }
    let total = nodes.len();
    if total > 1 {
        Ok(Node::Sequence(nodes))
    } else if total == 1 {
        Ok(nodes.pop().unwrap())
    } else {
        Err(ParserError::from_input(input))
    }
}

//

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
    fn test_parse_hardcoded_reset_keyword_wrapped_in_braces_markup() -> crate::Result<()>
    {
        let mut input = "{reset}";
        let result = parse_markup(&mut input);

        assert_equal!(result, Ok(Node::Reset));

        Ok(())
    }

    #[test]
    fn test_parse_reset_surrounded_by_arbitrary_strings() -> crate::Result<()> {
        let mut input = "hello {reset} world";
        let result = dbg!(parse_markup(&mut input));

        assert_equal!(
            result,
            Ok(Node::Sequence(vec![
                Node::String("hello ".to_string()),
                Node::Reset,
                Node::String(" world".to_string())
            ]))
        );

        Ok(())
    }

    #[test]
    fn test_parse_color_rgb_hex_with_hash_prefix() -> crate::Result<()> {
        let mut input = "hello {color:#1EBC73} world{reset}";
        let result = dbg!(parse_markup(&mut input));

        assert_equal!(
            result,
            Ok(Node::Sequence(vec![
                Node::String("hello ".to_string()),
                Node::Color("#1EBC73".parse::<crate::Color>()?),
                Node::String(" world".to_string()),
                Node::Reset,
            ]))
        );

        Ok(())
    }
}
