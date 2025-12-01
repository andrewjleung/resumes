pub mod reload;
pub mod resolution;
pub mod typst;

use bon::Builder;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::read_to_string;
use std::iter::once;

use merge::Merge;
use serde::Deserialize;
use serde::Serialize;

use crate::config::resolution::config_path;
use crate::config::resolution::default_config_path;
use crate::config::typst::TypstConfig;
use crate::prelude::*;
use crate::resume::ResumeFilterPredicate;
use crate::utils::path::current_dir;
use crate::utils::path::path_buf_from_str;

#[derive(Serialize, Deserialize, Default, Clone, Merge, Debug)]
pub struct WorkConfig {
    #[merge(strategy = merge::vec::append)]
    pub filters: Vec<ResumeFilterPredicate>,
}

#[derive(Serialize, Deserialize, Default, Clone, Merge, Debug)]
pub struct ProjectConfig {
    #[merge(strategy = merge::vec::append)]
    pub filters: Vec<ResumeFilterPredicate>,
}

// TODO: allow specifying multiple versions with different filters?
#[derive(Serialize, Deserialize, Default, Clone, Merge, Builder, Debug)]
pub struct Config {
    #[merge(strategy = merge::option::recurse)]
    pub typst: Option<TypstConfig>,

    #[merge(strategy = merge::option::recurse)]
    pub work: Option<WorkConfig>,

    #[merge(strategy = merge::option::recurse)]
    pub projects: Option<ProjectConfig>,

    #[merge(strategy = merge::option::overwrite_none)]
    artifact_title: Option<String>,

    #[merge(strategy = merge::bool::overwrite_false)]
    #[serde(default)]
    pub clean: bool,

    #[merge(strategy = merge::option::overwrite_none)]
    #[builder(with = |dir: &str| -> Result<_> { path_buf_from_str(dir).context("failed to canonicalize output directory path") })]
    output_dir: Option<PathBuf>,

    #[merge(strategy = merge::option::overwrite_none)]
    #[builder(with = |dir: &str| -> Result<_> { path_buf_from_str(dir).context("failed to canonicalize resume data path") })]
    resume_data_path: Option<PathBuf>,
}

impl Config {
    pub fn watched_file_paths(&self) -> impl IntoIterator<Item = PathBuf> {
        once(self.resume_data_path().ok())
            .chain(once(Some(
                self.typst.clone().unwrap_or_default().template(),
            )))
            .chain(once(config_path()))
            .flatten()
    }

    pub fn artifact_title(&self) -> String {
        self.artifact_title.clone().unwrap_or("resume".to_owned())
    }

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
