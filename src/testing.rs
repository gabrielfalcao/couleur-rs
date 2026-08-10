//! small set of utilities to unit test couleur_rs itself
use crate::{setup_logging, setup_tracing};
use std::sync::Once;

static INIT: Once = Once::new();

/// `global_setup` can be manually called before each unit test but
/// its effect only happens once.
///
/// It's  purpose is to generate a log file at `env!("CARGO_MANIFEST_DIR")/couleur.rs`
///
/// > NOTE: for this to work either the feature "tracing" or "logging" must be enabled.
#[cfg(any(feature = "logging", feature = "tracing"))]
pub fn global_setup() {
    INIT.call_once(|| setup_logging().or_else(|_| setup_tracing()).expect("setup logging/tracing"));
}

#[cfg(not(any(feature = "logging", feature = "tracing")))]
pub fn global_setup() {
    // NOOP
}
