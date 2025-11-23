use std::str::FromStr;

use camino::Utf8PathBuf as PathBuf;
use merge::Merge;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Merge, Default, Debug)]
pub struct TypstConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    pub template: Option<PathBuf>,
}

impl TypstConfig {
    pub fn template(&self) -> PathBuf {
        PathBuf::from_str("template.typ").expect("default typst template path is valid UTF-8")
    }
}
