use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

// use winnow::Parser;
use crate::{AnsiRenderable, Color, RenderableColor, Reset, ToAnsiEscSuffix, get_runtime_prefix};
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Node {
    Reset(Reset),
    Color(Color),
    Text(String),
    RenderableColor(RenderableColor),
    /// Sequence of Nodes
    Array(Vec<Node>),
    InvalidSyntax(String),
    /// End Of input
    EOI,
}

impl ToAnsiEscSuffix for Node {
    fn to_ansi_esc_suffix(&self) -> String {
        let rendered = match self {
            Node::Reset(reset) => reset.to_ansi_esc_suffix(),
            Node::Color(color) => color.to_ansi_esc_suffix(),
            Node::Text(string) => string.to_string(),
            Node::InvalidSyntax(string) => string.to_string(),
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
    Text,
    InvalidSyntax,
    RenderableColor,
    Array,
    EOI,
}
impl NodeType {
    pub fn name(&self) -> &'static str {
        match self {
            NodeType::Reset => "reset",
            NodeType::Color => "color",
            NodeType::Text => "text",
            NodeType::InvalidSyntax => "invalid_syntax",
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
            Node::Text(_) => NodeType::Text,
            Node::InvalidSyntax(_) => NodeType::InvalidSyntax,
            Node::RenderableColor(_) => NodeType::RenderableColor,
            Node::Array(_) => NodeType::Array,
            Node::EOI => NodeType::EOI,
        }
    }
    pub fn is_reset(&self) -> bool {
        self.node_type() == NodeType::Reset
    }
    pub fn is_color(&self) -> bool {
        self.node_type() == NodeType::Color
    }
    pub fn is_text(&self) -> bool {
        self.node_type() == NodeType::Text
    }
    pub fn is_invalid_syntax(&self) -> bool {
        self.node_type() == NodeType::InvalidSyntax
    }
    pub fn is_renderable_color(&self) -> bool {
        self.node_type() == NodeType::RenderableColor
    }
    pub fn is_array(&self) -> bool {
        self.node_type() == NodeType::Array
    }
    pub fn is_eoi(&self) -> bool {
        self.node_type() == NodeType::EOI
    }

    pub fn variant(&self) -> String {
        match self {
            Node::Reset(_) => "reset",
            Node::Color(_) => "color",
            Node::Text(_) => "text",
            Node::InvalidSyntax(_) => "invalid_syntax",
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
            Node::Text(_node) => vec![self.clone()],
            Node::InvalidSyntax(_node) => vec![self.clone()],
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
                Node::Text(text) => {
                    text.to_string()
                }
                Node::InvalidSyntax(invalid_syntax) => {
                    invalid_syntax.to_string()
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
    let nodes = nodes.fold(Vec::new(), |mut vec, node| match node.clone() {
        Node::Reset(_reset) => {
            vec.push(node.clone());
            vec
        }
        Node::Color(color) => {
            vec.push(Node::RenderableColor(RenderableColor::new(color)));
            vec
        }
        Node::Text(_string) => {
            vec.push(node.clone());
            vec
        }
        Node::InvalidSyntax(_string) => {
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
