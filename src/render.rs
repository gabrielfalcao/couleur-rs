use std::fmt::Display;

use winnow::error::ContextError;

use crate::{
    AnsiRenderable,
    Color,
    Contrast,
    Node,
    Prefix,
    RenderableColor,
    Reset,
    Result,
    ToAnsiEscSuffix,
    fold_nodes,
    // nodes,
    parse,
};

/// renders a given text after processing the template language
pub fn render<T: Display>(text: T, prefix: Prefix, add_reset_to_last_node: bool) -> Result<String> {
    let input = text.to_string().leak();
    let mut notes: Vec<Node> = {
        let result = parse::<String, ContextError>(input.to_string())?;
        let mut nodes = fold_nodes(result.to_vec().into_iter().map(|node| {
            if let Node::Color(color) = &node {
                Node::RenderableColor(RenderableColor::from(color))
            } else {
                node
            }
        }));
        if nodes.is_empty() {
            return Ok(text.to_string());
        }
        let last_node_does_not_reset_sequence =
            nodes.last().map(|node| !node.clone().is_reset()).unwrap_or_default();
        if last_node_does_not_reset_sequence && !add_reset_to_last_node {
            nodes.push(Node::Reset(Reset::new(prefix)));
        }
        nodes
    };
    let result = nodes.into_iter().map(|node| node.render()).collect::<String>();
    Ok(result)
}
