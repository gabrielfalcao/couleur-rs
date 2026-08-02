// use crate::{Error, Result};
use crate::{Color, Error, Result};

#[cfg(test)]
mod tests {
    /// # TDD TODO:
    ///
    /// ### first set of red -> green -> refactor rounds:
    ///
    /// 1. parse "{reset}" to `Node::Reset`
    /// 2. parse "{color:#F04F78}" to `Node::Color(crate::Color)`
    /// 3. parse "{color:F04F78}" to `Node::Color(crate::Color)`
    /// 4. parse "{color:240,79,120}" to `Node::Color(crate::Color)`
    /// 5. parse "{color:240,  79, 120 , }" to `Node::Color(crate::Color)`
    ///
    /// ### second set of red -> green -> refactor rounds:
    ///
    /// 1. parse "hello {reset} world" to `Node::Array(vec![Node::Text("hello "), Node::Reset, Node::Text(" world")])`
    /// 2. parse "{color:#4D9BE6}hello {color:#91DB69}world{reset}" to something like `Node::Array(vec![Node::Color("#4D9BE6".parse::<crate::Color>()?), Node::Text("hello "), Node::Color("#91DB69".parse::<crate::Color>()?), Node::Text("world"), Node::Reset])`
    ///
    /// ### third set of red -> green -> refactor rounds:
    ///
    /// 1. parse "{layer:bg}" to `Node::Layer(crate::Layer::BG)`
    /// 2. parse "{layer:fg}" to `Node::Layer(crate::Layer::FG)`
    /// 3. parse "{color:#F9C22B@layer:bg}" to `Node::AnsiLayered(Node::Layer(crate::Layer::FG), Node::Color("#F9C22B".parse<crate::Color>()?))`
    ///
    /// ### forth set of red -> green -> refactor rounds:
    ///
    /// 1. parse "{contrast:*VARIANT*}" for each of **variant** of the `crate::Contrast` enum, that is:
    ///   1.1 "{contrast:none}" should parse to `Node::Contrast(Contrast::None)`
    ///   1.2 "{contrast:read}" should parse to `Node::Contrast(Contrast::Read)`
    ///   1.3 "{contrast:high_bit}" should parse to `Node::Contrast(Contrast::HighBit)`
    ///   1.4 "{contrast:harmonic}" should parse to `Node::Contrast(Contrast::Harmonic)`
    ///   1.5 "{contrast:web}" should parse to `Node::Contrast(Contrast::Web)`
    ///
    /// 2. parse "{color:#E83B3B%contrast:web}" to `Node::ContrastedColor(Node::Contrast(Contrast::Web), Node::Color("#E83B3B".parse<crate::Color>()?))`
    /// 3. parse "{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World" to `Node::Array(vec![Node::Color(Node::Color("#E83B3B".parse<crate::Color>()?)), Node::Text("Hello"), Node::ContrastedColor(Node::Contrast(Contrast::Web), Node::Color("#E83B3B".parse<crate::Color>()?)), Node::Text(" World")])`
    ///   3.1 IMPORTANT: take note of this particular test spec and make a reference to it when writing tests for template rendering: "Hello" must be colored with #E83B3B while " World" must be colored with #68BBBB because that's its *"web"* contrast color.
    ///
    use super::*;
    use k9::assert_equal;

    #[test]
    fn test_parse_hardcoded_reset_keyword_wrapped_in_braces_markup() -> crate::Result<()> {
        let mut input = "{reset}";
        let result = dbg!(parse_markup(&mut input));

        assert_equal!(result, Ok(Node::Reset));

        Ok(())
    }

    //
    //
}
