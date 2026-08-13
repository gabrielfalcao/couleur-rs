use std::ops::{Deref, DerefMut};

use crate::Prefix;

pub struct PrefixContainer {
    pub prefix: Prefix,
}
impl PrefixContainer {
    pub fn set(&mut self, prefix: Prefix) {
        self.prefix = prefix;
    }
    pub fn get(&self) -> Prefix {
        self.prefix
    }
}
impl Default for PrefixContainer {
    fn default() -> PrefixContainer {
        let prefix = Prefix::default();
        PrefixContainer { prefix }
    }
}
impl Deref for PrefixContainer {
    type Target = Prefix;

    fn deref(&self) -> &Self::Target {
        &self.prefix
    }
}

impl DerefMut for PrefixContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.prefix
    }
}
