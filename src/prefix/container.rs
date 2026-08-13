use std::ops::{Deref, DerefMut};

use crate::Prefix;

pub struct PrefixContainer {
    pub prefix: Prefix,
}

impl<T> Deref for PrefixContainer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.prefix
    }
}

impl<T> DerefMut for PrefixContainer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.prefix
    }
}
