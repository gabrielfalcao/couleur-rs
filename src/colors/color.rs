use crate::ANSI256Color;
use crate::RGBColor;

pub enum Color {
    Rgb(RGBColor),
    ANSI256(ANSI256Color),
    ANSISystem(ANSISystemColor),
}
