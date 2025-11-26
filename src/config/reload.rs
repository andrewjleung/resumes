use merge::Merge;

use crate::config::{Config, load};

#[derive(Clone)]
pub struct ReloadableConfig {
    base: Config,
    pub current: Config,
}

impl ReloadableConfig {
    pub fn new(base: Config, secondary: Option<Config>) -> Self {
        let mut current = base.clone();

        if let Some(secondary) = secondary {
            current.merge(secondary);
        }

        current.merge(Config::default());
        ReloadableConfig { base, current }
    }

    pub fn reload(self) -> Self {
        Self::new(self.base, load())
    }
}

impl From<Config> for ReloadableConfig {
    fn from(value: Config) -> Self {
        Self::new(value, None)
    }
}

impl Iterator for ReloadableConfig {
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.clone().reload())
    }

    type Item = ReloadableConfig;
}
