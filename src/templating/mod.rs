pub mod node;
pub use node::Node;

pub mod types;
#[cfg(feature = "tracing")] pub use tracing::{Level, event, instrument, span};
#[cfg(feature = "tracing")] use tracing_subscriber::fmt::writer::EitherWriter;
pub use types::{Result, Stream};
use winnow::{
    ModalResult,
    Parser,
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
use {
    crate::{
        AnsiRenderable,
        Color,
        Contrast,
        Error,
        Layer,
        RenderableColor,
        Reset,
        Value,
        // ansi_renderable::{
        //     AnsiRenderable,
        //     AnsiRenderableWithColor,
        //     AnsiRenderableWithColorAndLayer,
        // },
        setup_logging,
    },
    std::fmt::{Debug, Display},
};

#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_node<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Node, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    if input.is_empty() {
        return Err(ErrMode::Cut(ParserError::from_input(input)));
    }
    // log::debug!("parse_node called with input: {input:#?}", &input);
    alt((
        renderable_color::<E>.map(Node::RenderableColor), // RenderableColor
        reset::<E>.map(Node::Reset),                      // Reset
        color::<E>.map(Node::Color),                      // Color
        layer::<E>.map(Node::Layer),                      // Layer
        contrast::<E>.map(Node::Contrast),                // Contrast
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
pub fn layer<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Layer, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);

    preceded('{', terminated(within_curly_braces::parse_layer::<E>, "}"))
        .context(StrContext::Expected("ansi rendering contrast".into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn contrast<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Contrast, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);

    preceded('{', terminated(within_curly_braces::parse_contrast::<E>, '}'))
        .context(StrContext::Expected("ansi rendering contrast".into()))
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
    mut input: &mut Stream<'i>,
) -> ModalResult<Node, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    let mut winnow_it = iterator(input, parse_node::<ContextError>);
    let res = winnow_it.map(|node| node).collect::<Vec<Node>>();

    winnow_it.finish();
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
    let mut input = input.to_string();
    let mut input: &'i mut str = input.leak();
    let result = nodes::<ContextError>
        .parse(input)
        .map_err(|e| Error::TemplateParseError(format!("{e}")))?;
    Ok(result)
}
pub fn render_nodes<T: AnsiRenderable, I: Iterator<Item = T>>(items: I) -> String {
    let p = items.map(|i| i.render()).collect::<String>();
    p
}

pub fn render<
    'i,
    I: Iterator<Item = T>,
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext> + Debug + Display,
    T: AnsiRenderable + Display,
>(
    input: T,
) -> crate::Result<String> {
    let mut input = input.to_string().leak();
    let resolve = nodes::<ContextError>
        .parse(input)
        .map_err(|error| Error::TemplateParseError(error.to_string()))?;
    Ok(resolve.render())
}

mod within_curly_braces {
    use std::{
        fmt::{Debug, Display},
        str::FromStr,
    };

    #[cfg(feature = "tracing")] pub use tracing::{Level, event, instrument, span};
    use winnow::{
        ModalResult,
        Parser,
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

    use super::*;
    use crate::{
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

    #[cfg_attr(feature = "tracing", instrument)]
    pub fn parse_contrast<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
        input: &mut Stream<'i>,
    ) -> ModalResult<Contrast, E> {
        use super::*;
        #[cfg(feature = "tracing")]
        span!(Level::TRACE, "input", input);
        preceded(
            "contrast:",
            alt((
                "none".value(Contrast::None),
                "read".value(Contrast::Read),
                "high_bit".value(Contrast::HighBit),
                "harmonic".value(Contrast::Harmonic),
                "web".value(Contrast::Web),
            )),
        )
        .parse_next(input)
    }
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn parse_color<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
        input: &mut Stream<'i>,
    ) -> ModalResult<Color, E> {
        #[cfg(feature = "tracing")]
        span!(Level::TRACE, "input", input);
        preceded("color:", alt((parse_rgb_triple, preceded("#", parse_rgb_hex), parse_rgb_hex)))
            .parse_next(input)
    }
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn parse_layer<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
        input: &mut Stream<'i>,
    ) -> ModalResult<Layer, E> {
        #[cfg(feature = "tracing")]
        span!(Level::TRACE, "input", input);
        preceded("layer:", alt(("bg".value(Layer::BG), "fg".value(Layer::FG)))).parse_next(input)
    }
}
