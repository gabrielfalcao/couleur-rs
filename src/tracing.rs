use crate::{Color, Contrast, Layer, Prefix, Reset};
use tracing::{
    Value,
    field::{Field, Visit},
};

impl Value for Layer {
    fn record(&self, key: &Field, visitor: &mut dyn Visit) {
        visitor.record_debug(key, self)
    }
}
impl Value for Prefix {
    fn record(&self, key: &Field, visitor: &mut dyn Visit) {
        visitor.record_debug(key, self)
    }
}
impl Value for Contrast {
    fn record(&self, key: &Field, visitor: &mut dyn Visit) {
        visitor.record_debug(key, self)
    }
}
impl Value for Color {
    fn record(&self, key: &Field, visitor: &mut dyn Visit) {
        visitor.record_debug(key, self)
    }
}
impl Value for Reset {
    fn record(&self, key: &Field, visitor: &mut dyn Visit) {
        visitor.record_debug(key, self)
    }
}
