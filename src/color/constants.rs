use crate::{Color, Value};
/// [`Color`] which holds the absolute black RGB color.
pub const BLACK: Color = Color(Value(0.0_f32), Value(0.0_f32), Value(0.0_f32));

/// [`Color`] which holds the absolute white RGB color.
pub const WHITE: Color = Color(Value(255.0_f32), Value(255.0_f32), Value(255.0_f32));
