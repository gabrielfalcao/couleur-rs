#[cfg(feature = "tracing")] use tracing::{Level, instrument, span};
use winnow::{
    ModalResult,
    Parser,
    ascii::{dec_uint, hex_digit1},
    combinator::{alt, preceded, repeat, terminated},
    error::{AddContext, ContextError, ErrMode, ParserError, StrContext},
    token::take_while,
};

use super::within_curly_braces;
use crate::{Color, Contrast, Error, Layer, Node, RenderableColor, Reset, Stream, ToAnsiEscSuffix};

#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_node<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Node, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    if input.is_empty() {
        return Err(ErrMode::Cut(ParserError::from_input(input)));
    }
    alt((
        renderable_color::<E>.map(Node::RenderableColor), // RenderableColor
        reset::<E>.map(Node::Reset),                      // Reset
        color::<E>.map(Node::Color),                      // Color
        text::<E>.map(Node::Text),                        // Text
        nodes::<E>,                                       // Array
    ))
    .parse_next(input)
}

#[cfg_attr(feature = "tracing", instrument)]
pub fn reset<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Reset, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    preceded('{', terminated("reset".value(Reset::default()), '}'))
        .context(StrContext::Expected("reset".into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn color<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    preceded('{', terminated(within_curly_braces::parse_color::<E>, '}'))
        .context(StrContext::Expected("rgb color".into()))
        .parse_next(input)
}

#[cfg_attr(feature = "tracing", instrument)]
pub fn renderable_color<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<RenderableColor, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    preceded(
        '{',
        terminated(
            alt((
                (
                    // Color + Layer + Contrast
                    within_curly_braces::parse_color::<E>,
                    preceded("@", within_curly_braces::parse_layer::<E>),
                    preceded("%", within_curly_braces::parse_contrast::<E>),
                )
                    .map(|(color, layer, contrast): (Color, Layer, Contrast)| {
                        RenderableColor::new(color).with_layer(layer).with_contrast(contrast)
                    }),
                (
                    // Color + Layer
                    within_curly_braces::parse_color::<E>,
                    preceded("@", within_curly_braces::parse_layer::<E>),
                )
                    .map(|(color, layer): (Color, Layer)| {
                        RenderableColor::new(color).with_layer(layer)
                    }),
                (
                    // Color + Contrast
                    within_curly_braces::parse_color::<E>,
                    preceded("%", within_curly_braces::parse_contrast::<E>),
                )
                    .map(|(color, contrast): (Color, Contrast)| {
                        RenderableColor::new(color).with_contrast(contrast)
                    }),
            )),
            '}',
        ),
    )
    .context(StrContext::Expected("rgb color".into()))
    .parse_next(input)
}

#[cfg_attr(feature = "tracing", instrument)]
pub fn text<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<String, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    alt((take_while(0.., |c: char| c != '{').context(StrContext::Expected("text".into())),))
        .parse_next(input)
        .map(|s| s.to_string())
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_u8<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<u8, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    dec_uint::<Stream<'i>, u8, ErrMode<E>>
        .context(StrContext::Expected("unsigned number between 0 and 255".into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_rgb_hex<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    repeat(0..5, hex_digit1)
        .fold(|| String::new(), |acc, item| format!("{acc}{item}"))
        .context(StrContext::Expected("6 hex digits".into()))
        .map(|string| string.parse::<Color>().expect("6 hex digits"))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_rgb_triple<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    parse_triple
        .map(|(red, green, blue)| Color::from_triple(red.into(), green.into(), blue.into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_triple<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<(u8, u8, u8), E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    (
        terminated(parse_u8, (ws, ',', ws)),
        terminated(parse_u8, (ws, ',', ws)),
        alt((terminated(parse_u8, (ws, ',', ws)), parse_u8)),
    )
        .context(StrContext::Expected("three comma-separated unsigned numbers".into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn ws<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<&'i str, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    take_while(0.., &[' ', '\t', '\r', '\n'])
        .context(StrContext::Expected("white space".into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn nodes<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Node, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);

    let initial_input = input.to_string();
    if initial_input.len() == 0 {
        #[cfg(any(feature = "logging", feature = "tracing"))]
        log::error!("empty input");
        return Ok(Node::EOI);
    }

    let mut res = Vec::<Node>::new();
    while input.len() > 0 {
        let parsed = parse_node::<E>(input)?;
        if parsed == Node::EOI {
            break;
        }
        res.push(parsed);
    }
    Ok(Node::Array(res))
}

#[cfg_attr(feature = "tracing", instrument)]
pub fn parse<
    'i,
    T: std::fmt::Debug + std::fmt::Display,
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext> + std::fmt::Display,
>(
    input: T,
) -> crate::Result<Node> {
    let input = input.to_string();
    let input: &'i mut str = input.leak();
    let result = nodes::<ContextError>
        .parse(input)
        .map_err(|e| Error::TemplateParseError(format!("{e}")))?;
    Ok(result)
}
pub fn render_nodes<T: ToAnsiEscSuffix, I: Iterator<Item = T>>(items: I) -> String {
    let p = items.map(|i| i.to_ansi_esc_suffix()).collect::<String>();
    p
}

// pub fn render<
//     'i,
//     I: Iterator<Item = T>,
//     E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext> + Debug + Display,
//     T: ToAnsiEscSuffix + Display,
// >(
//     input: T,
// ) -> crate::Result<String> {
//     let input = input.to_string().leak();
//     let resolve = nodes::<ContextError>
//         .parse(input)
//         .map_err(|error| Error::TemplateParseError(error.to_string()))?;
//     Ok(resolve.render())
// }
