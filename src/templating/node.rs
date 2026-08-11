use std::fmt::{Debug, Display};

use winnow::Parser;

use crate::{Color, Contrast, Layer, RenderableColor, Reset, ToAnsiEscSuffix};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Reset(Reset),
    Color(Color),
    Layer(Layer),
    Contrast(Contrast),
    Text(String),
    RenderableColor(RenderableColor),
    Array(Vec<Node>),
    // End Of input
    EOI,
}
impl Node {
    pub fn render(&self) -> String {
        let rendered = match self {
            Node::Reset(reset) => reset.render(),
            Node::Color(color) => color.render(),
            Node::Layer(layer) => layer.render(),
            Node::Contrast(contrast) => contrast.render(),
            Node::Text(string) => string.render(),
            Node::RenderableColor(renderable_color) => renderable_color.render(),
            // Node::Array(array_of_value) => array_of_value.render(),
            Node::Array(nodes) => {
                nodes.iter().map(|n| n.render()).collect::<Vec<String>>().join("")
            }
            Node::EOI => String::new(),
        };
        [rendered, Reset::default().render()].into_iter().collect::<String>()
    }
    pub fn variant(&self) -> String {
        match self {
            Node::Reset(_) => "reset",
            Node::Color(_) => "color",
            Node::Layer(_) => "layer",
            Node::Contrast(_) => "contrast",
            Node::Text(_) => "string",
            Node::RenderableColor(_) => "renderable_color",
            Node::Array(_) => "array_of_value",
            Node::EOI => "end_of_input",
            // Node::Array(nodes) => {
            //     nodes.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",")
            // }
        }
        .to_string()
    }
    pub fn to_vec(&self) -> Vec<Node> {
        match self {
            Node::Reset(_node) => vec![self.clone()],
            Node::Color(_node) => vec![self.clone()],
            Node::Layer(_node) => vec![self.clone()],
            Node::Contrast(_node) => vec![self.clone()],
            Node::Text(_node) => vec![self.clone()],
            Node::RenderableColor(_node) => vec![self.clone()],
            Node::Array(nodes) => nodes.to_vec(),
            Node::EOI => Vec::new(),
        }
    }
}

impl ToAnsiEscSuffix for Node {
    fn to_ansi_esc_suffix(&self) -> String {
        match self {
            Node::Reset(node) => node.render(),           // node.reset(),
            Node::Color(node) => node.render(),           // node.color(),
            Node::Layer(node) => node.render(),           // node.layer(),
            Node::Contrast(node) => node.render(),        // node.contrast(),
            Node::Text(node) => node.to_string(),         // node.to()_string(),
            Node::RenderableColor(node) => node.render(), // node.render(),
            Node::Array(node) => node.render(), // node.iter().map(|node| node.render()).collect::<String>(),
            Node::EOI => String::new(),
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
                    nodes.iter().map(|node| ToAnsiEscSuffix::render(node)).collect::<String>()
                }
                Node::EOI => {
                    String::new()
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
