use std::{
    any::type_name_of_val,
    fmt::{Debug, Display},
    iter::{IntoIterator, Iterator},
};

use crate::{Prefix, ToAnsiEscSuffix, templating::Node};

/// The [`AnsiRenderable`] trait allow [`Prefix-aware`] implementors
/// that implement the [`ToAnsiEscSuffix`] trait in order to produce a
/// full ANSI sequence.
///
/// [`ColorPalette::name`]: crate::state::ColorPalette::name
/// [`Color`]: crate::Color
/// [`Contrast`]: crate::Contrast
/// [`Layer`]: crate::Layer
/// [`Prefix`]: crate::Prefix
/// [`Prefix-aware`]: crate::Prefix
/// [`RenderableColor`]: crate::RenderableColor
/// [`Reset`]: crate::Reset
/// [`ToAnsiEscSuffix`]: crate::ToAnsiEscSuffix
pub trait AnsiRenderable: ToAnsiEscSuffix + Debug {
    /// This method must return the [`Prefix`] contained in the
    /// implementor, then the [`render()`] method can combine the prefix
    /// with the suffix provided by
    /// [`ToAnsiEscSuffix::to_ansi_esc_suffix()`] resulting in a full
    /// ANSI sequence.
    ///
    /// [`Prefix`]: crate::Prefix
    /// [`render()`]: crate::AnsiRenderable::render
    /// [`ToAnsiEscSuffix`]: crate::ToAnsiEscSuffix
    /// [`ToAnsiEscSuffix::to_ansi_esc_suffix()`]: crate::ToAnsiEscSuffix::to_ansi_esc_suffix
    fn prefix(&self) -> String;
    fn render(&self) -> String {
        let prefix = self.prefix();
        let suffix = self.to_ansi_esc_suffix();
        // dbg!(&self, type_name_of_val(&self), &prefix, &suffix);
        [prefix, suffix].into_iter().collect::<String>()
    }
}

impl<T> ToAnsiEscSuffix for (Prefix, T)
where
    T: ToAnsiEscSuffix + Clone + Display + Debug,
{
    fn to_ansi_esc_suffix(&self) -> String {
        let (_, suffix) = self.clone();
        suffix.to_ansi_esc_suffix()
    }
}

impl<T> AnsiRenderable for (Prefix, T)
where
    T: ToAnsiEscSuffix + Clone + Display + Debug,
{
    fn prefix(&self) -> String {
        let (prefix, _) = self.clone();
        format!("{prefix}")
    }
}
