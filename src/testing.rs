use {
    crate::{setup_logging, setup_tracing},
    std::sync::Once,
};

static INIT: Once = Once::new();

pub(crate) fn global_setup() {
    INIT.call_once(|| setup_logging().or_else(|_| setup_tracing()).expect("setup logging/tracing"));
}
