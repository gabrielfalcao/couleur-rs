use crate::Value;
use std::cmp::{max, min};

pub fn max_rgb(r: Value, g: Value, b: Value) -> Value {
    let rg_max = if r > g { r } else { g };
    if rg_max > b { rg_max } else { b }
}

pub fn min_rgb(r: Value, g: Value, b: Value) -> Value {
    let rg_min = if r < g { r } else { g };
    if rg_min < b { rg_min } else { b }
}
