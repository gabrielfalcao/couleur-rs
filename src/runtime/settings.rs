use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{Color, RuntimeColors};
use log::LevelFilter;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct RuntimeSettings {
    ///
    pub(crate) log_level: LevelFilter,

    /// container with named colors and color palettes
    pub(crate) colors: RuntimeColors,
    pub(crate) query_timeout: Duration,
}

impl Default for RuntimeSettings {
    fn default() -> RuntimeSettings {
        let log_level = log::max_level();
        let query_timeout = Terminal::MINIMUM_QUERY_TIMEOUT;
        let colors = RuntimeColors::default();
        RuntimeSettings { log_level, query_timeout, colors }
    }
}
impl RuntimeSettings {
    /// sets the new logging level and returns the previous value
    pub fn set_logging_level(&mut self, level: LevelFilter) -> Option<LevelFilter> {
        let previous = self.log_level.clone();
        self.log_level = Some(level);
        log::set_max_level(level);
    }
    pub fn logging_level(&self) -> LevelFilter {
        self.log_level
    }
    pub fn set_query_timeout(&mut self, timeout: Duration) {
        self.query_timeout = timeout;
    }
    pub fn query_timeout(&self) -> Duration {
        self.query_timeout
    }
    pub fn query_options(&self) -> QueryOptions {
        QueryOptions {timeout: self.query_timeout}
    }

    /// TODO: unit test
    pub fn set_fallback_background_color(&mut self, color: Color) {
        self.colors.set_fallback_bg_color(color);
    }
    pub fn fallback_background_color(&self) -> Color {
        self.colors.fallback_bg_color
    }
    pub fn set_fallback_foreground_color(&mut self, color: Color) {
        self.colors.set_fallback_fg_color(color);
    }
    pub fn fallback_foreground_color(&self) -> Color {
        self.colors.fallback_fg_color
    }
}
