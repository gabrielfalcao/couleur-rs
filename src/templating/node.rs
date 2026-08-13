use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

// use winnow::Parser;
use crate::{
    AnsiRenderable,
    Color,
    Contrast,
    Layer,
    RenderableColor,
    Reset,
    ToAnsiEscSuffix,
    get_runtime_prefix,
};
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Node {
    Reset(Reset),
    Color(Color),
    Layer(Layer),
    Contrast(Contrast),
    Text(String),
    RenderableColor(RenderableColor),
    /// Sequence of Nodes
    Array(Vec<Node>),
    /// End Of input
    EOI,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_yaml::to_string(&self).unwrap())
    }
}
impl ToAnsiEscSuffix for Node {
    fn to_ansi_esc_suffix(&self) -> String {
        let rendered = match self {
            Node::Reset(reset) => reset.to_ansi_esc_suffix(),
            Node::Color(color) => color.to_ansi_esc_suffix(),
            Node::Layer(layer) => layer.to_ansi_esc_suffix(),
            Node::Contrast(contrast) => contrast.to_ansi_esc_suffix(),
            Node::Text(string) => string.to_string(),
            Node::RenderableColor(renderable_color) => renderable_color.to_ansi_esc_suffix(),
            // Node::Array(array_of_value) => array_of_value.to_ansi_esc_suffix(),
            Node::Array(nodes) => {
                nodes.iter().map(|n| n.to_ansi_esc_suffix()).collect::<Vec<String>>().join("")
            }
            Node::EOI => String::new(),
        };
        [rendered].into_iter().collect::<String>()
    }
}
impl AnsiRenderable for Node {
    fn prefix(&self) -> String {
        if let Node::Text(_) = self.clone() {
            String::new()
        } else {
            get_runtime_prefix().to_string()
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NodeType {
    Reset,
    Color,
    Layer,
    Contrast,
    Text,
    RenderableColor,
    Array,
    EOI,
}
impl NodeType {
    pub fn name(&self) -> &'static str {
        match self {
            NodeType::Reset => "reset",
            NodeType::Color => "color",
            NodeType::Layer => "layer",
            NodeType::Contrast => "contrast",
            NodeType::Text => "text",
            NodeType::RenderableColor => "renderable_color",
            NodeType::Array => "array",
            NodeType::EOI => "eoi",
        }
    }
}
impl Node {
    pub fn node_type(&self) -> NodeType {
        match self {
            Node::Reset(_) => NodeType::Reset,
            Node::Color(_) => NodeType::Color,
            Node::Layer(_) => NodeType::Layer,
            Node::Contrast(_) => NodeType::Contrast,
            Node::Text(_) => NodeType::Text,
            Node::RenderableColor(_) => NodeType::RenderableColor,
            Node::Array(_) => NodeType::Array,
            Node::Array(_) => NodeType::Array,
            Node::EOI => NodeType::EOI,
        }
    }
    pub fn is_reset(self) -> bool {
        self.node_type() == NodeType::Reset
    }
    pub fn is_color(self) -> bool {
        self.node_type() == NodeType::Color
    }
    pub fn is_layer(self) -> bool {
        self.node_type() == NodeType::Layer
    }
    pub fn is_contrast(self) -> bool {
        self.node_type() == NodeType::Contrast
    }
    pub fn is_text(self) -> bool {
        self.node_type() == NodeType::Text
    }
    pub fn is_renderablecolor(&self) -> bool {
        self.node_type() == NodeType::RenderableColor
    }
    pub fn is_array(self) -> bool {
        self.node_type() == NodeType::Array
    }
    pub fn is_eoi(&self) -> bool {
        self.node_type() == NodeType::EOI
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
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Node::{variant}(data) => {repr:#?}",
            variant = self.variant(),
            repr = match self {
                Node::Reset(reset) => {
                    reset.to_ansi_esc_suffix()
                }
                Node::Color(color) => {
                    color.to_ansi_esc_suffix()
                }
                Node::Layer(layer) => {
                    layer.to_ansi_esc_suffix()
                }
                Node::Contrast(contrast) => {
                    contrast.to_ansi_esc_suffix()
                }
                Node::Text(text) => {
                    text.to_string()
                }
                Node::RenderableColor(color) => {
                    color.to_string()
                }
                Node::Array(nodes) => {
                    nodes.iter().map(|node| node.to_ansi_esc_suffix()).collect::<String>()
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
pub fn fold_nodes<T: Iterator<Item = Node>>(nodes: T) -> Vec<Node> {
    let mut nodes = nodes.fold(Vec::new(), |mut vec, node| match node.clone() {
        Node::Reset(_reset) => {
            vec.push(node.clone());
            vec
        }
        Node::Color(color) => {
            vec.push(Node::RenderableColor(RenderableColor::new(color)));
            vec
        }
        Node::Layer(_layer) => {
            vec.push(node.clone());
            vec
        }
        Node::Contrast(_contrast) => {
            vec.push(node.clone());
            vec
        }
        Node::Text(_string) => {
            vec.push(node.clone());
            vec
        }
        Node::RenderableColor(_renderable_color) => {
            vec.push(node.clone());
            vec
        }
        Node::Array(nodes) => {
            vec.extend(fold_nodes(nodes.into_iter()));
            vec
        }
        Node::EOI => vec,
    });
    nodes
}
#[cfg(test)]
mod node_tests {
    use crate::{
        AnsiRenderable,
        Color,
        Error,
        Layer,
        Node,
        Prefix,
        RenderableColor,
        Reset,
        Result,
        ToAnsiEscSuffix,
    };
    #[test]
    fn test_renderable_color() -> Result<()> {
        let color = RenderableColor::new("#F9C22B".parse::<Color>()?);
        let node = Node::RenderableColor(color);
        assert_eq!(color.to_ansi_esc_suffix(), "38;2;249;194;43m");
        assert_eq!(node.to_ansi_esc_suffix(), "38;2;249;194;43m");
        assert_eq!(color.render(), "\x1b[38;2;249;194;43m");
        assert_eq!(node.render(), "\x1b[38;2;249;194;43m");
        Ok(())
    }
    #[test]
    fn test_text() -> Result<()> {
        let text = format!("hello world");
        let node = Node::Text(text.clone());
        assert_eq!(&text, "hello world");
        assert_eq!(node.to_ansi_esc_suffix(), "hello world");
        assert_eq!(node.render(), "hello world");
        Ok(())
    }
    #[test]
    fn test_reset() -> Result<()> {
        let color = Reset::new(Prefix::default());
        let node = Node::Reset(color);
        assert_eq!(color.to_ansi_esc_suffix(), "0m");
        assert_eq!(node.to_ansi_esc_suffix(), "0m");
        assert_eq!(color.render(), "\x1b[0m");
        assert_eq!(node.render(), "\x1b[0m");
        Ok(())
    }
}
