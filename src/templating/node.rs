use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

// use winnow::Parser;
use crate::{AnsiRenderable, Color, Contrast, Layer, RenderableColor, Reset, ToAnsiEscSuffix};
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
            crate::PREFIX.get_mut().get().to_string()
        }
    }
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
    nodes.fold(Vec::new(), |mut vec, node| match node.clone() {
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
    })
}

#[cfg(test)]
mod node_tests {
    use crate::{
        AnsiRenderable,
        Color,
        Error,
        Layer,
        Node,
        RenderableColor,
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
}
