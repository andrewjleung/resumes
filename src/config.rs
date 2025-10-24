use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::read_to_string;

use anyhow::{Context, Result};
use camino::Utf8Path as Path;
use camino::Utf8PathBuf as PathBuf;
use serde::Deserialize;
use serde::Serialize;

use crate::resume::ResumeFilterPredicate;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct WorkConfig {
    pub filters: Option<Vec<ResumeFilterPredicate>>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ProjectConfig {
    pub filters: Option<Vec<ResumeFilterPredicate>>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub work: Option<WorkConfig>,
    pub projects: Option<ProjectConfig>,
}

pub fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .map(|dir| {
            PathBuf::from_path_buf(dir.join(".config/reze/reze.toml"))
                .expect("config path should be UTF-8")
        })
        .expect("home directory should exist")
}

impl Config {
    pub fn reload(&mut self) -> Result<()> {
        let reloaded = config().context("failed to reload config")?;

        if let Some(reloaded) = reloaded {
            self.work = reloaded.work;
            self.projects = reloaded.projects;
        }

        Ok(())
    }
}

#[allow(dead_code)]
fn tilde_expand(dir: &Path) -> PathBuf {
    Path::new(&shellexpand::tilde(dir.as_str())).to_path_buf()
}

pub fn init() -> Result<bool> {
    let config_path = default_config_path();

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

pub fn config() -> Result<Option<Config>> {
    let config_path = default_config_path();

    if !fs::exists(&config_path).context("failed to check for existence of config path")? {
        return Ok(None);
    }

    Ok(Some(
        File::open(config_path)
            .context("failed to open config")
            .and_then(|file| read_to_string(file).context("failed to read config"))
            .and_then(|content| {
                toml::from_str(&content).context("failed to parse config into TOML")
            })?,
    ))
}
