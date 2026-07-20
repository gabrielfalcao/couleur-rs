#![allow(unused)]
use couleur_rs::{AnsiColorizer, Color, Contrast, Layer, Reset, Result, Value, Wrap};
use k9::assert_equal;
use std::cmp::{max, min};

#[test]
fn test_colorize_fg() -> Result<()> {
    let bg = None;
    let bold = true;
    let fg = Some("#FFCC00".parse::<Color>()?);
    let contrast = Contrast::None;
    let reset = Reset::After;
    let wrap = Wrap::Before;
    let colorizer = AnsiColorizer {
        bg,
        fg,
        contrast,
        wrap,
        bold,
        reset,
    };
    let result = colorizer.colorize("test 123")?;
    assert_equal!(
        result,
        "\x1b[0m\x1b[1;48;2;28;26;28m\x1b[1;38;2;255;204;0mtest 123\x1b[0m"
    );
    Ok(())
}
