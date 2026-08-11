use std::{
    fmt::Display,
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
/// [`RenderableColor`]: crate::RenderableColor
/// [`Reset`]: crate::Reset
/// [`ToAnsiEscSuffix`]: crate::ToAnsiEscSuffix
pub trait AnsiRenderable: ToAnsiEscSuffix {
    /// This method must return the [`Prefix`] contained in the
    /// implementor, then the [`render()`] method can combine the prefix
    /// with the suffix provided by
    /// [`ToAnsiEscSuffix::to_ansi_esc_suffix()`] resulting in a full
    /// ANSI sequence.
    ///
    /// [`Prefix`]: crate::Prefix
    /// [`render()`]: crate::ToAnsiEscSuffix::render
    /// [`ToAnsiEscSuffix`]: crate::ToAnsiEscSuffix
    /// [`ToAnsiEscSuffix::to_ansi_esc_suffix()`]: crate::ToAnsiEscSuffix::to_ansi_esc_suffix
    fn prefix(&self) -> String;
    fn render(&self) -> String {
        [self.prefix(), self.to_ansi_esc_suffix()].into_iter().collect::<String>()
    }
}

impl<T> ToAnsiEscSuffix for (Prefix, T)
where
    T: ToAnsiEscSuffix + Clone + Display,
{
    fn to_ansi_esc_suffix(&self) -> String {
        let (_, suffix) = self.clone();
        suffix.to_ansi_esc_suffix()
    }
}

impl<T> AnsiRenderable for (Prefix, T)
where
    T: ToAnsiEscSuffix + Clone + Display,
{
    fn prefix(&self) -> String {
        let (prefix, _) = self.clone();
        format!("prefix")

    }
}
