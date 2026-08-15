use crate::U8Triple;

pub fn convert_ansi256_to_rgb_triple(ansi: u8) -> U8Triple {
    match ansi {
        0 => (0u8, 0u8, 0u8),
        1 => (205u8, 0u8, 0u8),
        2 => (0u8, 205u8, 0u8),
        3 => (205u8, 205u8, 0u8),
        4 => (0u8, 0u8, 205u8),
        5 => (205u8, 0u8, 205u8),
        6 => (0u8, 205u8, 205u8),
        7 => (229u8, 229u8, 229u8),
        8 => (127u8, 127u8, 127u8),
        9 => (255u8, 0u8, 0u8),
        10 => (0u8, 255u8, 0u8),
        11 => (255u8, 255u8, 0u8),
        12 => (0u8, 0u8, 255u8),
        13 => (255u8, 0u8, 255u8),
        14 => (0u8, 255u8, 255u8),
        15 => (255u8, 255u8, 255u8),

        // 6x6x6 Color Cube (16-231)
        16..=231 => {
            let code = ansi - 16u8;
            let r = code / 36u8;
            let g = (code / 6u8) % 6u8;
            let b = code % 6u8;

            // Map the 0-5 range into 0-255.
            // 0 -> 0, 1 -> 51, 2 -> 102, 3 -> 153, 4 -> 204, 5 -> 255
            let map = |val| if val == 0 { 0u8 } else { val * 40u8 + 15u8 };

            (map(r), map(g), map(b))
        }

        // Grayscale Ramp (232-255)
        // 24 steps, starting slightly brighter than black (8) to slightly darker than white (238)
        232..=255 => {
            let level = 8u8 + (ansi - 232u8) * 10u8;
            (level, level, level)
        }
    }
}
