use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[cfg(feature = "tracing")] use tracing::{Level, event, instrument, span};
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
    ToAnsiEscSuffix,
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
