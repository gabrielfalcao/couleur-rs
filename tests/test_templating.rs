#![allow(unused)]
use std::cmp::{max, min};

use {
    couleur_rs::{Reset, render_template},
    k9::assert_equal,
};

#[test]
fn test_render_foreground() -> Result<()> {
    assert_equal!(
        render(
            "{color:#FB6B1D}warning: {color:#F9C22B}this is only the foreground",
            Reset::Around
        )?,
        "\x1b[0m\x1b[1;38;2;251;107;29mwarning: \x1b[1;38;2;249;194;43mthis is only the foreground\x1b[0m"
    );
    Ok(())
}

#[test]
fn test_render_foreground_explicit() -> Result<()> {
    assert_equal!(
        render(
            "{color:#FB6B1D,layer:fg}warning: {color:#F9C22B,layer:foreground}this is only the foreground",
            Reset::Around
        )?,
        "\x1b[0m\x1b[1;38;2;251;107;29mwarning: \x1b[1;38;2;249;194;43mthis is only the foreground\x1b[0m"
    );
    Ok(())
}

#[test]
fn test_render_background_explicit() -> Result<()> {
    assert_equal!(
        render(
            "{color:#FB6B1D,layer:bg}warning: {color:#F9C22B,layer:background}this is only the background",
            Reset::Around
        )?,
        "\x1b[0m\x1b[1;38;2;251;107;29mwarning: \x1b[1;38;2;249;194;43mthis is only the background\x1b[0m"
    );
    Ok(())
}

#[test]
fn test_render_terminal_background_color_on_foreground_and_light_color_on_background() -> Result<()>
{
    assert_equal!(
        render(
            "{color:terminal_background,layer:fg}{color:#90E162,layer:bg}NEON VIBES",
            Reset::Around
        )?,
        "\x1b[0m\x1b[38;2;28;26;28m\x1b[48;2;144;225;98mNEON VIBES\x1b[0m"
    );
    Ok(())
}

#[test]
fn test_render_terminal_foreground_color_on_background_and_dark_color_on_foreground() -> Result<()>
{
    assert_equal!(
        render(
            "{color:terminal_foreground,layer:bg}{color:#000000,layer:fg}might look a bit weird",
            Reset::Around
        )?,
        "\x1b[0m\x1b[48;5;207;198;166m[38;2;0;0;0mmight look a bit weird\x1b[0m"
    );
    Ok(())
}
