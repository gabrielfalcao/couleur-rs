use crate::{Error, Layer, Result, Rule};
use pest::{Span, iterators::Pair};
use from_pest::FromPest;

fn span_into_str<'a>(span: Span<'a>) -> &'a str {
    span.as_str()
}
fn span_into_string(span: Span) -> String {
    span_into_str(span).to_string()
}

// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::field))]
// pub struct Field {
//     #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
//     pub value: f64,
// }

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::rgb_hex))]
pub struct RgbHex {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::color_rgb))]
pub struct ColorRgb {
    pub hex: RgbHex,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::color))]
pub struct Color {
    pub rgb: ColorRgb,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::markup))]
pub struct Markup {
    pub color: Color,
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::unhandled))]
pub struct Unhandled {
    #[pest_ast(outer(with(span_into_string)))]
    pub string: String,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::node))]
pub enum Node {
    Markup(Markup),
    Unhandled(Unhandled),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::text))]
pub struct Text {
    pub nodes: Vec<Node>,
    _eoi: Eoi,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi;
