pub mod resolution;
pub mod typst;

use bon::Builder;
use schemars::JsonSchema;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::read_to_string;

use merge::Merge;
use serde::Deserialize;
use serde::Serialize;

use crate::config::resolution::config_path;
use crate::config::resolution::default_config_path;
use crate::config::typst::TypstConfig;
use crate::prelude::*;
use crate::resume::query::Clause;
use crate::utils::path::current_dir;
use crate::utils::path::path_buf_from_str;

const DEFAULT_TITLE: &str = "resume";

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Title(pub String);

impl Default for Title {
    fn default() -> Self {
        Title(DEFAULT_TITLE.to_string())
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Title(s) = self;
        s.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Merge, Builder, Debug, JsonSchema)]
pub struct Config {
    #[merge(strategy = merge::option::recurse)]
    pub typst: Option<TypstConfig>,

    #[merge(strategy = merge::hashmap::overwrite)]
    #[serde(default)]
    #[builder(default)]
    pub queries: HashMap<String, Vec<Clause>>,

    #[merge(strategy = merge::option::overwrite_none)]
    pub title: Option<Title>,

    #[merge(strategy = merge::bool::overwrite_false)]
    #[serde(default)]
    pub clean: bool,

    #[merge(strategy = merge::option::overwrite_none)]
    #[builder(with = |dir: &str| -> Result<_> { path_buf_from_str(dir).context("failed to canonicalize output directory path") })]
    #[schemars(with = "String")]
    output_dir: Option<PathBuf>,

    #[merge(strategy = merge::option::overwrite_none)]
    #[builder(with = |dir: &str| -> Result<_> { path_buf_from_str(dir).context("failed to canonicalize resume data path") })]
    #[schemars(with = "String")]
    resume_data_path: Option<PathBuf>,
}

impl Config {
    pub fn output_dir(&self) -> Result<PathBuf> {
        match self.output_dir.clone() {
            Some(dir) => Ok(dir),
            None => current_dir(),
        }
    }

    pub fn resume_data_path(&self) -> Result<PathBuf> {
        Ok(match self.resume_data_path.clone() {
            Some(dir) => dir,
            None => current_dir()?.join("resume.json"),
        })
    }
}

#[allow(dead_code)]
pub fn init() -> Result<bool> {
    let config_path = default_config_path()?;

    if fs::exists(&config_path).context("failed to check for existence of config path")? {
        return Ok(false);
    }

    fs::DirBuilder::new()
        .recursive(true)
        .create(config_path.parent().unwrap())
        .context("failed to create config path")?;

    let content = toml::to_string_pretty(&Config::default())
        .context("failed to serialize initial config into TOML")?;

    File::create_new(&config_path)
        .context("failed to create config file")?
        .write_all(content.as_bytes())
        .context("failed to write initial config to config file")?;

    Ok(true)
}

pub fn load() -> Result<Config> {
    Ok(config_path().unwrap_or_default())
        .and_then(|path| File::open(path).context("failed to open config"))
        .and_then(|file| read_to_string(file).context("failed to read config"))
        .and_then(|content| toml::from_str(&content).context("failed to parse config into TOML"))
}
