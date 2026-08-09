// use crate::{Error, Result};
#[cfg(feature = "tracing")] pub use tracing::{Level, event, instrument, span};
#[cfg(feature = "tracing")] use tracing_subscriber::fmt::writer::EitherWriter;
use {
    crate::{
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
    },
    std::fmt::{Debug, Display},
};

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
pub(crate) type Stream<'i> = &'i str;

impl AnsiRenderable for Node {
    fn render(&self) -> String {
        match self {
            Node::Reset(node) => node.render(),
            Node::Color(node) => node.render(),
            Node::Layer(node) => node.render(),
            Node::Contrast(node) => node.render(),
            Node::Text(node) => node.to_string(),
            Node::RenderableColor(node) => node.render(),
            Node::Array(node) => node.iter().map(|node| node.render()).collect::<String>(),
        }
    }
}
// impl AnsiRenderable for Node {
//     fn render(&self) -> String {
//         let nodes = match self {
//             Node::Reset(reset) => {
//                 vec![reset.render()]
//             }
//             Node::Color(color) => {
//                 vec![color]
//             }
//             Node::Layer(layer) => {
//                 vec![layer]
//             }
//             Node::Contrast(contrast) => {
//                 vec![contrast]
//             }
//             Node::Text(text) => {
//                 vec![text]
//             }
//             Node::RenderableColor(color) => {
//                 vec![color]
//             }
//             Node::Array(nodes) => {
//                 nodes.iter().map(|node| AnsiRenderable::render(node)).collect::<String>()
//             }
//         };
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Reset(Reset),
    Color(Color),
    Layer(Layer),
    Contrast(Contrast),
    Text(String),
    RenderableColor(RenderableColor),
    Array(Vec<Node>),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Node::{variant}(data) => {repr:#?}",
            variant = self.variant(),
            repr = match self {
                Node::Reset(reset) => {
                    reset.render()
                }
                Node::Color(color) => {
                    color.render()
                }
                Node::Layer(layer) => {
                    layer.render()
                }
                Node::Contrast(contrast) => {
                    contrast.render()
                }
                Node::Text(text) => {
                    text.to_string()
                }
                Node::RenderableColor(color) => {
                    color.to_string()
                }
                Node::Array(nodes) => {
                    nodes.iter().map(|node| AnsiRenderable::render(node)).collect::<String>()
                }
            }
        )
    }
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
fn reset<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Reset, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    preceded('{', terminated("reset".value(Reset::default()), '}'))
        .context(StrContext::Expected("reset".into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
fn color<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    preceded('{', terminated(within_curly_braces::parse_color::<E>, '}'))
        .context(StrContext::Expected("rgb color".into()))
        .parse_next(input)
}

#[cfg_attr(feature = "tracing", instrument)]
fn renderable_color<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
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
fn layer<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Layer, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);

    preceded('{', terminated(within_curly_braces::parse_layer::<E>, "}"))
        .context(StrContext::Expected("ansi rendering contrast".into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
fn contrast<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Contrast, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);

    preceded('{', terminated(within_curly_braces::parse_contrast::<E>, '}'))
        .context(StrContext::Expected("ansi rendering contrast".into()))
        .parse_next(input)
}

impl Node {
    pub fn variant(&self) -> String {
        match self {
            Node::Reset(_) => "reset",
            Node::Color(_) => "color",
            Node::Layer(_) => "layer",
            Node::Contrast(_) => "contrast",
            Node::Text(_) => "string",
            Node::RenderableColor(_) => "renderable_color",
            Node::Array(_) => "arrry_of_value",
            // Node::Array(nodes) => {
            //     nodes.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",")
            // }
        }
        .to_string()
    }
}
mod within_curly_braces {
    #[cfg(feature = "tracing")] pub use tracing::{Level, event, instrument, span};
    use {
        super::*,
        crate::{
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
        },
        std::{
            fmt::{Debug, Display},
            str::FromStr,
        },
        winnow::{
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
        },
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

#[cfg_attr(feature = "tracing", instrument)]
fn text<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<String, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    alt((take_while(0.., |c: char| c != '{').context(StrContext::Expected("text".into())),))
        .parse_next(input)
        .map(|s| s.to_string())
}
#[cfg_attr(feature = "tracing", instrument)]
fn parse_u8<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<u8, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    dec_uint::<Stream<'i>, u8, ErrMode<E>>
        .context(StrContext::Expected("unsigned number between 0 and 255".into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
fn parse_rgb_hex<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
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
fn parse_rgb_triple<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<Color, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    parse_triple
        .map(|(red, green, blue)| Color::from_triple(red.into(), green.into(), blue.into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
fn parse_triple<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
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
fn ws<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    input: &mut Stream<'i>,
) -> ModalResult<&'i str, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    take_while(0.., &[' ', '\t', '\r', '\n'])
        .context(StrContext::Expected("white space".into()))
        .parse_next(input)
}
#[cfg_attr(feature = "tracing", instrument)]
fn nodes<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
    mut input: &mut Stream<'i>,
) -> ModalResult<Node, E> {
    #[cfg(feature = "tracing")]
    span!(Level::TRACE, "input", input);
    let mut winnow_it = iterator(input, parse_node::<E>);
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
) -> Result<Node> {
    let mut input: &'i mut str = input.to_string().leak();
    let result = nodes::<E>.parse(input).map_err(|e| Error::TemplateParseError(format!("{e}")))?;
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
    T: AnsiRenderable,
>(
    input: T,
) -> Result<String> {
    let resolve = nodes::<Error>.parse(input)?;
    Ok(resolve)
}

// pub fn render<'i, T: std::fmt::Display + std::fmt::Debug>(input: T) -> Result<String> {
//     let mut input: &'i mut str = input.to_string().leak();
//     Ok(render::<ContextError>(input)?)
// }
#[test]
fn test_parse_renderable_color_with_contrast() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{color:#E83B3B%contrast:web}"),
        Ok((
            "",
            Node::RenderableColor(
                RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
                    .with_contrast(Contrast::Web)
            )
        ))
    );

    Ok(())
}
#[test]
fn test_parse_renderable_color_with_layer_and_contrast() -> Result<()> {
    assert_eq!(
        parse_node::<Error>.parse_peek("{color:#E83B3B@layer:bg%contrast:web}"),
        Ok((
            "",
            Node::RenderableColor(
                RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
                    .with_layer(Layer::BG)
                    .with_contrast(Contrast::Web)
            )
        ))
    );

    Ok(())
}
// #[test]
// fn test_parse_renderable_color_with_layer_contrast_and_text() -> Result<()> {
//     assert_eq!(
//         nodes::<Error>.parse_peek("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
//         Ok((
//             "",
//             Node::Array(vec![
//                 Node::Color("#E83B3B".parse::<crate::Color>().unwrap()),
//                 Node::Text("Hello".to_string()),
//                 Node::RenderableColor(
//                     RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
//                         .with_contrast(Contrast::Web)
//                 ),
//                 Node::Text(" World".to_string())
//             ])
//         ))
//     );
//     assert_eq!(
//         parse::<&str, Error>("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
//         Ok(Node::Array(vec![
//             Node::Color("#E83B3B".parse::<crate::Color>().unwrap()),
//             Node::Text("Hello".to_string()),
//             Node::RenderableColor(
//                 RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
//                     .with_contrast(Contrast::Web)
//             ),
//             Node::Text(" World".to_string())
//         ]))
//     );
//
//     Ok(())
// }
// #[test]
// fn test_render_error() -> Result<()> {
//     assert_eq!(render("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"), Ok(format!("")));
//     Ok(())
// }
