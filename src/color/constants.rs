use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    str::FromStr,
    sync::LazyLock,
};

use super::Color;
/// static instance of [`Color`] which holds the absolute black RGB color.
pub static BLACK: LazyLock<Color> =
    LazyLock::new(|| Color::new(0.0_f32, 0.0_f32, 0.0_f32).unwrap());

/// static instance of [`Color`] which holds the absolute white RGB color.
pub static WHITE: LazyLock<Color> =
    LazyLock::new(|| Color::new(255.0_f32, 255.0_f32, 255.0_f32).unwrap());
