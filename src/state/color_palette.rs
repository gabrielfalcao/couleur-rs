use crate::{Color, Error, Result};
use heck::ToTitleCase;
use iocore::Path;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// [`ColorPalette`] is an object which stores a color palette based
/// on a [`name`](`ColorPalette::name`), a set of [`RGB
/// Colors`](`crate::Color`) and, optionally, a mapping of [`color
/// names`](`ColorPalette::color_names`) whose key is a [`Color`] and
/// the value is a [`String`].
///
/// Its primary role is to hold a mapping of [`ColorPalette`] objects
/// by palette name. In other words, the key of the
/// [`Context::palettes`] property is exactly the same value as the
/// [`ColorPalette::name`] property of [`ColorPalette`].
///
/// [`ColorPalette`]: crate::state::ColorPalette
/// [`ColorPalette::name`]: crate::state::ColorPalette::name
/// [`Context`]: crate::state::Context
/// [`Context::palettes`]: crate::state::Context::palettes
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Serialize, Deserialize)]
pub struct ColorPalette {
    name: String,
    colors: BTreeSet<Color>,
    color_names: Option<BTreeMap<Color, String>>,
}

impl ColorPalette {
    /// WIP
    pub fn from_lospec_hex_filename<T: Into<Path>>(path: T) -> Result<ColorPalette> {
        let path = path.into();
        let name = path.without_extension().name().to_title_case();
        Err(Error::RenderError(format!("{path} {name}")))
    }
}
