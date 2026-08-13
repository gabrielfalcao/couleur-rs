#[cfg(feature = "tracing")] use tracing::{Level, instrument, span};
use winnow::{
    ModalResult,
    Parser,
    combinator::{alt, preceded},
    error::{AddContext, ParserError, StrContext},
};

use super::*;
use crate::{Color, Contrast, Layer, Stream};

#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_contrast<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Contrast, E> {
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
    alt((preceded(
        "contrasted_color:",
        (
            alt((parse_rgb_triple, preceded("#", parse_rgb_hex), parse_rgb_hex)),
            preceded(
                ":",
                alt((
                    "none".value(Contrast::None),
                    "read".value(Contrast::Read),
                    "high_bit".value(Contrast::HighBit),
                    "harmonic".value(Contrast::Harmonic),
                    "web".value(Contrast::Web),
                )),
            ),
        )
            .map(|(color, contrast): (Color, Contrast)| contrast.of(color)),
        preceded("color:", alt((parse_rgb_triple, preceded("#", parse_rgb_hex), parse_rgb_hex))),
    )))
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
