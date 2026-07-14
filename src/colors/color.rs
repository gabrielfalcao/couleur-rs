use crate::RGBColor;
use crate::ANSI256Color;

pub enum Color {
    Rgb(RGBColor),
    ANSI256(ANSI256Color),
    ANSISystem(ANSISystemColor),
}
