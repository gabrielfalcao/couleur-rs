use crate::RGBValue;
use std::cmp::{max, min};

pub fn max_rgb(r: RGBValue, g: RGBValue, b: RGBValue) -> RGBValue {
    let rg_max = if r > g { r } else { g };
    if rg_max > b { rg_max } else { b }
}

pub fn min_rgb(r: RGBValue, g: RGBValue, b: RGBValue) -> RGBValue {
    let rg_min = if r < g { r } else { g };
    if rg_min < b { rg_min } else { b }
}
