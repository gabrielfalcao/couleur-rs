// use crate::{Error, Result};
use std::fmt::{Debug, Display};
#[cfg(feature = "tracing")] pub(self) use tracing::{Level, event, instrument, span};
pub(self) use {
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
    std::str::FromStr,
};

pub(self) use winnow::{
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
        .context(StrContext::Expected("ansi rendering layer".into()))
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

mod within_curly_braces {
    use super::*;
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
        use super::*;
        #[cfg(feature = "tracing")]
        span!(Level::TRACE, "input", input);
        preceded("color:", alt((parse_rgb_triple, preceded("#", parse_rgb_hex), parse_rgb_hex)))
            .parse_next(input)
    }
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn parse_layer<'i, E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext>>(
        input: &mut Stream<'i>,
    ) -> ModalResult<Layer, E> {
        use super::*;
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
    items.map(|i| i.render()).collect::<String>()
}
pub fn render<
    'i,
    E: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext> + Display + Debug,
    T: Display + Debug,
>(
    input: T,
) -> Result<String> {
    let nodes = parse(input)?;
    let items = render_nodes([nodes].into_iter()).to_string();
    Ok(render::<E>(input)?)
}
/// # TDD TODO:
///
/// ### first set of red -> green -> refactor rounds:
///
///  - [x] parse "{reset}" to `Node::Reset` *DONE:
///  - [x] 2. parse "{color:#F04F78}" to `Node::Color(crate::Color)`
///  - [x] 3. parse "{color:F04F78}" to `Node::Color(crate::Color)`
///  - [x] 4. parse "{color:240,79,120}" to `Node::Color(crate::Color)`
///  - [x] 5. parse "{color:240,  79, 120 , }" to `Node::Color(crate::Color)`
///
/// ### second set of red -> green -> refactor rounds:
///
///  - [x] 1. parse "hello {reset} world" to `Node::Array(vec![Node::Text("hello "), Node::Reset, Node::Text(" world")])`
///  - [x] 2. parse "{color:#4D9BE6}hello {color:#91DB69}world{reset}" to something like `Node::Array(vec![Node::Color("#4D9BE6".parse::<crate::Color>()?), Node::Text("hello "), Node::Color("#91DB69".parse::<crate::Color>()?), Node::Text("world"), Node::Reset])`
///
/// ### third set of red -> green -> refactor rounds:
///
///  - [x] 1. parse "{layer:bg}" to `Node::Layer(crate::Layer::BG)`
///  - [x] 2. parse "{layer:fg}" to `Node::Layer(crate::Layer::FG)`
///  - [x] 3. parse "{color:#F9C22B@layer:bg}" to something like `Node::RenderableColor(Node::Layer(crate::Layer::FG), Node::Color("#F9C22B".parse<crate::Color>()?))`
///
/// ### fourth set of red -> green -> refactor rounds:
///
///  - [x] 1. parse "{contrast:*VARIANT*}" for each of **variant** of the `crate::Contrast` enum, that is:
///    - [x] 1.1 "{contrast:none}" should parse to `Node::Contrast(Contrast::None)`
///    - [x] 1.2 "{contrast:read}" should parse to `Node::Contrast(Contrast::Read)`
///    - [x] 1.3 "{contrast:high_bit}" should parse to `Node::Contrast(Contrast::HighBit)`
///    - [x] 1.4 "{contrast:harmonic}" should parse to `Node::Contrast(Contrast::Harmonic)`
///    - [x] 1.5 "{contrast:web}" should parse to `Node::Contrast(Contrast::Web)`
///
///  - [x] 2. parse "{color:#E83B3B%contrast:web}" to something like `Node::RenderableColor(Node::Contrast(Contrast::Web), Node::Color("#E83B3B".parse<crate::Color>()?))`
///  - [x] 3. parse "{color:#E83B3B@layer:bg%contrast:web}" to something like `Node::RenderableColor(Node::Color("#E83B3B".parse<crate::Color>()?, Node::Contrast(Contrast::Web), Node::Layer(Layer::BG), ))`
///  - [x] 4. parse "{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World" to something like `Node::Array(vec![Node::Color(Node::Color("#E83B3B".parse<crate::Color>()?)), Node::Text("Hello"), Node::RenderableColor(Node::Contrast(Contrast::Web), Node::Color("#E83B3B".parse<crate::Color>()?)), Node::Text(" World")])`
///    - [ ] 4.1 IMPORTANT: take note of this particular test spec and make a reference to it when writing tests for template rendering: "Hello" must be colored with #E83B3B while " World" must be colored with #68BBBB because that's its *"web"* contrast color.
/// 4.0 IMPORTANT: take note of this particular test spec and make a reference to it when writing tests for template rendering: "Hello" must be colored with #E83B3B while " World" must be colored with #68BBBB because that's its *"web"* contrast color.
