use std::iter::{IntoIterator, Iterator};

use crate::Node;

/// The [`ToAnsiEscSuffix`] trait provides a way for elements of this
/// crate to produce a string which represents one element of an [ANSI
/// escape sequence](https://en.wikipedia.org/wiki/ANSI_escape_code)
/// that needs to be prefixed with a [`Prefix`](crate::Prefix) in
/// order to render a whole sequence.
///
/// Some crate elements such as [`Color`] must be
/// concatenated with other elements in order to generate a full ANSI
/// sequence. Still taking [`Color`] as an example, a
/// [`Layer`] has to be provided in order to determine
/// whether to colorize the foreground or background of a text.
///
/// The element [`RenderableColor`] is a struct that contains
/// [`Color`], [`Layer`], [`Prefix`] and [`Contrast`]. Its
/// implementation of the [`ToAnsiEscSuffix`] trait makes it able to
/// generate a full ANSI sequence.
///
/// Finally, the [`Reset`] element represents the ANSI
/// escape code which clears all coloring when rendered after colored
/// text (hence the name reset)
///
/// [`ColorPalette::name`]: crate::state::ColorPalette::name
/// [`Prefix`]: crate::Prefix
/// [`Color`]: crate::Color
/// [`Contrast`]: crate::Contrast
/// [`Layer`]: crate::Layer
/// [`RenderableColor`]: crate::RenderableColor
/// [`Reset`]: crate::Reset
/// [`ToAnsiEscSuffix`]: crate::ToAnsiEscSuffix
pub trait ToAnsiEscSuffix: Sized + Clone {
    fn render(&self) -> String {
        self.to_ansi_esc_suffix()
    }
    // fn to_ansi(&self, prefix: Option<Prefix>) -> String {
    //     format!(
    //         "{prefix}{code}",
    //         prefix = prefix.unwrap_or_default(),
    //         code = self.to_ansi_esc_suffix()
    //     )
    // }

    fn to_ansi_esc_suffix(&self) -> String;
}

// impl<T> ToAnsiEscSuffix for T
// where
//     T: std::ops::Deref<Target = str> + Clone + Debug + Display,
// {
//     fn render(&self) -> String {
//         self.to_string()
//     }
// }
impl ToAnsiEscSuffix for &str {
    fn to_ansi_esc_suffix(&self) -> String {
        self.to_string()
    }
}
impl ToAnsiEscSuffix for String {
    fn to_ansi_esc_suffix(&self) -> String {
        self.to_string()
    }
}

impl ToAnsiEscSuffix for &Vec<Node> {
    fn to_ansi_esc_suffix(&self) -> String {
        self.into_iter().map(|node| node.render()).collect::<String>()
    }
}
