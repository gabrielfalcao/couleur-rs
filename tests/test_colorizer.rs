#![allow(unused)]
use std::cmp::{max, min};

use couleur_rs::{AnsiColorizer, Color, Contrast, Layer, Prefix, Reset, Result, Value, Wrap};
use k9::assert_equal;

fn make_simple_test_colorizer(bg: Option<Color>, fg: Option<Color>, prefix: Option<Prefix>) -> AnsiColorizer {
    let contrast = Contrast::None;
    let reset = Reset::After;
    let wrap = Wrap::Before;
    let colorizer = AnsiColorizer { bg, fg, contrast, wrap, prefix, reset };
    colorizer
}

fn make_prefix_test_colorizer(prefix: Option<Prefix>) -> AnsiColorizer {
    let colorizer = make_simple_test_colorizer(None, Some("#FFCC00".parse::<Color>().unwrap()), prefix);
    colorizer
}

#[test]
fn test_colorize_fg_none_prefix() -> Result<()> {
    let colorizer = make_prefix_test_colorizer(None);
    assert_equal!(colorizer.colorize("test 123")?, "\x1b[0m\x1b[48;2;28;26;28m\x1b[38;2;255;204;0mtest 123\x1b[0m");
    Ok(())
}

#[test]
fn test_colorize_fg_hex_prefix() -> Result<()> {
    let colorizer = make_prefix_test_colorizer(Some(Prefix::Hex));
    assert_equal!(colorizer.colorize("test 123")?, "\x1b[0m\x1b[48;2;28;26;28m\x1b[38;2;255;204;0mtest 123\x1b[0m");
    Ok(())
}

#[test]
fn test_colorize_fg_unicode_prefix() -> Result<()> {
    let colorizer = make_prefix_test_colorizer(Some(Prefix::Unicode));
    assert_equal!(colorizer.colorize("test 123")?, "\u{1b}[0m\u{1b}[48;2;28;26;28m\u{1b}[38;2;255;204;0mtest 123\u{1b}[0m");
    Ok(())
}

#[test]
fn test_colorize_fg_octal_prefix() -> Result<()> {
    let colorizer = make_prefix_test_colorizer(Some(Prefix::Octal));
    assert_equal!(colorizer.colorize("test 123")?, "\033[0m\033[48;2;28;26;28m\033[38;2;255;204;0mtest 123\033[0m");
    Ok(())
}

#[test]
fn test_colorize_fg_escape_prefix() -> Result<()> {
    let colorizer = make_prefix_test_colorizer(Some(Prefix::Escape));
    assert_equal!(colorizer.colorize("test 123")?, r"\E[0m\E[48;2;28;26;28m\E[38;2;255;204;0mtest 123\E[0m");
    Ok(())
}
