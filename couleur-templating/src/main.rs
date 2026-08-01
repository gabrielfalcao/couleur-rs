#![allow(unused)]
use couleur_templating::{
    Color,
    Contrast,
    Definition,
    Error,
    InvalidMarkupToken,
    Layer,
    Node,
    PaletteColor,
    Reset,
    Result,
    parse_tokens,
};

fn main() -> Result<()> {
    let input = "{color:#FB6B1D}".to_string();
    let node = parse_tokens(&input)?;

    println!("node: {node:#?}");
    Ok(())
}
