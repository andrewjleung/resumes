use merge::Merge;

use crate::config::config::{Config, load};

#[derive(Clone)]
pub struct ReloadableConfig {
    base: Config,
    pub current: Config,
}

impl ReloadableConfig {
    pub fn new(base: Config, secondary: Option<Config>) -> Self {
        match secondary {
            Some(secondary) => {
                let mut current = base.clone();
                current.merge(secondary);
                current.merge(Config::default());
                ReloadableConfig { base, current }
            }
            None => ReloadableConfig {
                current: base.clone(),
                base,
            },
        }
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
