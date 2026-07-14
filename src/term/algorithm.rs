use std::fmt::Display;
use clap::{ValueEnum, builder::PossibleValue};

#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    Read,
    HighBit,
    Harmonic,
    Web,
}
impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.variant_name(),
        )
    }
}

impl Algorithm {
    pub fn variant_name(&self) -> &'static str {
        match self {
            Algorithm::Read => "read",
            Algorithm::HighBit => "high_bit",
            Algorithm::Harmonic => "harmonic",
            Algorithm::Web => "web",
        }
    }

    pub fn variants<'a>() -> &'a [Algorithm] {
        &[
            Algorithm::Read,
            Algorithm::HighBit,
            Algorithm::Harmonic,
            Algorithm::Web,
        ]
    }
}

impl ValueEnum for Algorithm {
    fn value_variants<'a>() -> &'a [Algorithm] {
        Algorithm::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<Algorithm, String> {
        let val = if ignore_case {
            val.to_lowercase()
        } else {
            val.to_string()
        };
        let val = val.trim();
        for (vmethod, smet) in Algorithm::variants().iter().map(|m| {
            (
                m,
                if ignore_case {
                    m.variant_name().to_lowercase().to_string()
                } else {
                    m.variant_name().to_string()
                },
            )
        }) {
            if val == smet {
                return Ok(vmethod.clone());
            }
        }
        return Err(val.to_string());
    }
}
