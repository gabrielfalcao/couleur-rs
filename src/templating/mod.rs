pub mod node;
pub use node::{Node, fold_nodes};
pub mod types;
pub use types::{Result, Stream};

#[doc(hidden)] pub mod parsers;
#[doc(inline)]
pub use parsers::functions::{
    color,
    nodes,
    parse,
    parse_node,
    parse_rgb_hex,
    parse_rgb_triple,
    parse_triple,
    parse_u8,
    render_nodes,
    renderable_color,
    reset,
    text,
    ws,
};
#[doc(inline)]
pub use parsers::within_curly_braces::{parse_color, parse_contrast, parse_layer};
