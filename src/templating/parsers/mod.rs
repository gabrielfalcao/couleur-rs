pub mod node;
pub use node::Node;

pub mod types;
pub use types::{Result, Stream};

pub mod functions;
pub use functions::{
    Node,
    Result,
    Stream,
    color,
    contrast,
    layer,
    nodes,
    parse,
    parse_node,
    parse_rgb_hex,
    parse_rgb_triple,
    parse_triple,
    parse_u8,
    render,
    render_nodes,
    renderable_color,
    reset,
    text,
    ws,
};

pub(crate) mod within_curly_braces;
pub use within_curly_braces::{
    parse_color,
    parse_contrast,
    parse_layer,
};
