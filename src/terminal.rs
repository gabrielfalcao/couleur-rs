use crate::{Color, Error, Layer, Result};
use terminal_colorsaurus::{QueryOptions, background_color, foreground_color};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub struct Terminal;
impl Terminal {
    pub fn background_color() -> Result<Color> {
        let terminal_bg_color = background_color(QueryOptions::default())?;
        let (r, g, b) = terminal_bg_color.scale_to_8bit();
        Ok(Color::from_triple(r.into(), g.into(), b.into()))
    }
    pub fn foreground_color() -> Result<Color> {
        let terminal_bg_color = foreground_color(QueryOptions::default())?;
        let (r, g, b) = terminal_bg_color.scale_to_8bit();
        Ok(Color::from_triple(r.into(), g.into(), b.into()))
    }
    pub fn layer_color(layer: Layer) -> Result<Color> {
        Ok(match layer {
            Layer::BG => Self::background_color()?,
            Layer::FG => Self::foreground_color()?,
        })
    }
    pub fn background_luminance() -> Result<f32> {
        let background_color = Self::background_color()?;
        Ok(background_color.get_binary_luminance())
    }
    pub fn foreground_luminance() -> Result<f32> {
        let foreground_color = Self::foreground_color()?;
        Ok(foreground_color.get_binary_luminance())
    }
    pub fn is_dark() -> Result<bool> {
        let luminance = Self::background_luminance()?;
        Ok(luminance < 128.0)
    }
    pub fn is_light() -> Result<bool> {
        let luminance = Self::background_luminance()?;
        Ok(luminance >= 128.0)
    }
}

// I1tkZXJpdmUoQ2xvbmUsIENvcHksIERlYnVnLCBQYXJ0aWFsT3JkLCBQYXJ0aWFsRXEsIEVxLCBPcmQpXQpwdWIgc3RydWN0IFRlcm1pbmFsSW5mbyB7CiAgICBwdWIgYmFja2dyb3VuZDogQ29sb3IsCiAgICBwdWIgZm9yZWdyb3VuZDogQ29sb3IsCiAgICBwdWIgaXNfZGFyazogYm9vbCwKICAgIHB1YiBpc19saWdodDogYm9vbCwKICAgIHB1YiBsdW1pbmFuY2U6IGYzMiwKfQo=
