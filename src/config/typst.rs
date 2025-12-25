use std::str::FromStr;

use crate::prelude::*;
use crate::utils::path::path_buf_from_str;
use bon::Builder;
use camino::Utf8PathBuf as PathBuf;
use merge::Merge;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

const DEFAULT_TEMPLATE: &str = "template.typ";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Template(pub PathBuf);

impl Default for Template {
    fn default() -> Self {
        Template(
            PathBuf::from_str(DEFAULT_TEMPLATE)
                .expect("default typst template path is valid UTF-8"),
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Merge, Default, Debug, Builder, JsonSchema)]
pub struct TypstConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    #[builder(with = |dir: &str| -> Result<_> { path_buf_from_str(dir).context("failed to canonicalize output directory path").map(Template) })]
    #[schemars(with = "String")]
    pub template: Option<Template>,
}
