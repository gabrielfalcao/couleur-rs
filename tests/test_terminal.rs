use couleur_rs::{Color, Error, Result, Terminal};

#[test]
fn test_terminal_luminance_functions() -> Result<()> {
    assert_eq!(Terminal::is_dark()?, true);
    assert_eq!(Terminal::is_light()?, false);
    assert_eq!(Terminal::background_luminance()?, 26.825998);
    assert_eq!(Terminal::foreground_luminance()?, 197.043);
    Ok(())
}
