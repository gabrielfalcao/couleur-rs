#[cfg(feature = "tracing")] pub use tracing::{Level, event, instrument, span};
#[cfg(feature = "tracing")] use tracing_subscriber::fmt::writer::EitherWriter;
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
type Result<T> = std::result::Result<T, ContextError>;
pub(crate) type Stream<'i> = &'i str;

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
impl Node {
    pub fn render(&self) -> String {
        match self {
            Node::Reset(reset) => reset.render(),
            Node::Color(color) => color.render(),
            Node::Layer(layer) => layer.render(),
            Node::Contrast(contrast) => contrast.render(),
            Node::Text(string) => string.render(),
            Node::RenderableColor(renderable_color) => renderable_color.render(),
            // Node::Array(arrry_of_value) => arrry_of_value.render(),
            Node::Array(nodes) => {
                nodes.iter().map(|n| n.render()).collect::<Vec<String>>().join("")
            }
        }
    }
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

impl AnsiRenderable for Node {
    fn render(&self, prefix: Option<Prefix>) -> String {
        match self {
            Node::Reset(node) => node.render(prefix),           // node.reset(),
            Node::Color(node) => node.render(prefix),           // node.color(),
            Node::Layer(node) => node.render(prefix),           // node.layer(),
            Node::Contrast(node) => node.render(prefix),        // node.contrast(),
            Node::Text(node) => node.to_string(),               // node.to()_string(),
            Node::RenderableColor(node) => node.render(prefix), // node.render(prefix),
            Node::Array(node) => node.render(prefix),           // node.iter().map(|node| node.render(prefix)).collect::<String>(),
        }
    }
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
