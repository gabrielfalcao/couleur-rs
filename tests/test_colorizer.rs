use couleur_rs::{Algorithm, Colorizer, Layer, RGBColor, RGBValue, Reset, Result, Wrap};
use k9::assert_equal;
use std::cmp::{max, min};

#[test]
fn test_colorize_fg() -> Result<()> {
    let bg = None;
    let bold = true;
    let fg = Some("#FFCC00".parse::<RGBColor>()?);
    let contrast = Algorithm::None;
    let reset = Reset::After;
    let wrap = Wrap::Before;
    let colorizer = Colorizer {
        bg,
        fg,
        contrast,
        wrap,
        bold,
        reset,
    };
    let result = colorizer.colorize("test 123")?;
    assert_equal!(result, "\x1b[1;38;2;255;204;0mtest 123\x1b[0m");
    Ok(())
}
