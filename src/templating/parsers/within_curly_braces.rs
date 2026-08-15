#[cfg(feature = "tracing")] use tracing::{Level, instrument, span};
use winnow::{
    ModalResult,
    Parser,
    combinator::{alt, cut_err, preceded, separated_pair},
    error::{AddContext, ParserError, StrContext},
};

use super::*;
use crate::{Color, Contrast, Layer, RenderableColor, Reset, Stream};

#[cfg_attr(feature = "tracing", instrument)]
pub fn rgb_color_declaration<
    'i,
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    alt((parse_rgb_triple, preceded("#", parse_rgb_hex), parse_rgb_hex)).parse_next(input)
}

#[cfg_attr(feature = "tracing", instrument)]
pub fn contrast_alternatives<
    'i,
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
>(
    input: &mut Stream<'i>,
) -> ModalResult<Contrast, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    alt((
        "none".value(Contrast::None),
        "read".value(Contrast::Read),
        "high_bit".value(Contrast::HighBit),
        "harmonic".value(Contrast::Harmonic),
        "web".value(Contrast::Web),
    ))
    .parse_next(input)
}

#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_contrast<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Contrast, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    preceded("contrast:", contrast_alternatives::<E>).parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_color<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    alt((
        preceded(
            "contrasted_color:",
            separated_pair(rgb_color_declaration::<E>, ":", contrast_alternatives::<E>)
                .map(|(color, contrast): (Color, Contrast)| contrast.of(color)),
        ),
        preceded("color:", rgb_color_declaration::<E>),
    ))
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
#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_reset<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Reset, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    "reset".value(Reset::default()).parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
pub fn parse_renderable_color<
    'i,
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>,
>(
    input: &mut Stream<'i>,
) -> ModalResult<RenderableColor, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
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
            .map(|(color, layer): (Color, Layer)| RenderableColor::new(color).with_layer(layer)),
        (
            // Color + Contrast
            within_curly_braces::parse_color::<E>,
            preceded("%", within_curly_braces::parse_contrast::<E>),
        )
            .map(|(color, contrast): (Color, Contrast)| {
                RenderableColor::new(color).with_contrast(contrast)
            }),
    ))
    .context(StrContext::Expected("rgb color".into()))
    .parse_next(input)
}
