#![allow(unused)]
use couleur_rs::{Terminal, Color, Result, Error};
use k9::assert_equal;

#[test]
fn test_terminal_luminance_functions() -> Result<()> {

    assert_equal!(Terminal::is_dark()?, true);
    assert_equal!(Terminal::is_light()?, false);
    assert_equal!(Terminal::background_luminance()?, 26.825998);
    assert_equal!(Terminal::foreground_luminance()?, 197.043);
    Ok(())
}
