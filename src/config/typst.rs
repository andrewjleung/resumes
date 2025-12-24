use std::str::FromStr;

use crate::prelude::*;
use crate::utils::path::path_buf_from_str;
use bon::Builder;
use camino::Utf8PathBuf as PathBuf;
use merge::Merge;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Merge, Default, Debug, Builder, JsonSchema)]
pub struct TypstConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    #[builder(with = |dir: &str| -> Result<_> { path_buf_from_str(dir).context("failed to canonicalize output directory path") })]
    #[schemars(with = "String")]
    template: Option<PathBuf>,
}

impl TypstConfig {
    pub fn template(&self) -> PathBuf {
        self.template.clone().unwrap_or(
            PathBuf::from_str("template.typ").expect("default typst template path is valid UTF-8"),
        )
    }
}
