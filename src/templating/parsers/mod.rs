#[doc(hidden)] pub mod functions;
#[doc(inline)]
pub use functions::{
    color,
    invalid_syntax,
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
    parse_within_curly_braces,
    ws,
};

#[doc(hidden)] pub mod within_curly_braces;
#[doc(inline)]
pub use within_curly_braces::{
    contrast_alternatives,
    parse_color,
    parse_contrast,
    parse_layer,
    parse_renderable_color,
    parse_reset,
    rgb_color_declaration,
};
