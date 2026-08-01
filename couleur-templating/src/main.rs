pub use couleur_templating::{
    Color,
    ColorRgb,
    Contrast,
    Definition,
    Error,
    Layer,
    Markup,
    Node,
    Reset,
    Result,
    RgbHex,
    Text,
    Unhandled,
    parse_tokens,
};

fn main() -> Result<()>{
    let input = "{color:#FB6B1D}".to_string();
    let node = parse_tokens(&input)?;

    println!("node: {node:#?}");
    Ok(())
}
