// examples:
//
// "hello {reset} world"
// {color:#FB6B1D,layer:bg}
// {color:terminal_foreground,layer:bg}
// {color:resurrect-64:cold_green_medium,layer:bg}
use crate::{Error, Result};
use nom::{
    IResult,
    Parser,
    branch::permutation,
    bytes::{is_not, tag},
    character::{
        anychar,
        complete::{alpha1, char, digit1},
    },
    combinator::map_res,
    error::{FromExternalError, ParseError},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated},
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node<'a> {
    List(Vec<Node<'a>>),
    Text(&'a str),
    String(String),
    Reset,
    None,
}

// pub fn parse_markup<'a, T, E: ParseError<T>>(input: &str) -> IResult<&str, Node<'a>> {
//     let result = delimited(char::<&str, E>('{'), is_not::<&str, E>("}"), char::<&str, E>('}')).parse(input);
//     dbg!(&result);
//
//     Ok((input, Node::None))
// }

fn parse_markup<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, Error>,
{
    // let within_braces = many1(is_not("}")); // many1 returns vec
    let within_braces = is_not("}");

    let mut braces_container = delimited(char('{'), within_braces, char('}'));

    // let parse_markup = map_res(braces_container, move |item| Node::List(item));

    // parse_markup(input)
    braces_container.parse(input).map(|(input, res)| (input, res.to_string()))
}

pub fn parse_nodes<'a>(input: &str) -> IResult<&str, Node<'a>> {
    let (rest, (input, node)) =
        permutation((parse_markup, many0(anychar))).parse(input).map(|(rest, (input, chars))| (rest, (input, Node::String(chars.into_iter().collect::<String>()))))?;

    dbg!(&rest, &input, &node);
    Ok((rest, node))
}

#[test]
fn test_hello_world() -> Result<()> {
    // // assert_eq!(parse_nodes("hello {reset} world")?, Ok(("", vec![Node::Text("hello "), Node::Reset, Node::Text(" world"),])))
    // parse_nodes("hello {reset} world")?;
    parse_markup("hello {reset} world")?;
    Ok(())
}
