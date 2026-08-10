use crate::{
    AnsiRenderable,
    Color,
    Contrast,
    Layer,
    RenderableColor,
    Reset,
};
use std::fmt::{Debug, Display};

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

impl AnsiRenderable for Node {
    fn render(&self) -> String {
        match self {
            Node::Reset(node) => node.render(),           // node.reset(),
            Node::Color(node) => node.render(),           // node.color(),
            Node::Layer(node) => node.render(),           // node.layer(),
            Node::Contrast(node) => node.render(),        // node.contrast(),
            Node::Text(node) => node.to_string(),         // node.to()_string(),
            Node::RenderableColor(node) => node.render(), // node.render(),
            Node::Array(node) => node.render(), // node.iter().map(|node| node.render()).collect::<String>(),
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
