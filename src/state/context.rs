use {
    crate::{ColorPalette, Error, Result},
    serde::{Deserialize, Serialize},
    std::collections::{BTreeMap, BTreeSet},
};

/// [`Context`] is an object containing state for template
/// rendering. It can be serialized and deserialized to load from
/// disk.
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
pub struct Context {
    /// the `palettes` property maps the name of a [`ColorPalette`]
    /// object to the object itself.
    ///
    /// For example, this map can be easily initializd from an
    /// [`Iterator<Item=ColoPalette>`] with:
    ///
    /// ```
    /// let palettes: BTreeMap<String, ColorPalette> = BTreeMap::from_iter(
    ///     palettes_iter.map(|palette| (palette.name.to_string(), palette.clone())),
    /// );
    /// ```
    pub palettes: BTreeMap<String, ColorPalette>,
}
