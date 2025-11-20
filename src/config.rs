use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::read_to_string;

use anyhow::{Context, Result, anyhow};
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

// TODO: allow specifying multiple versions with different filters?
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub work: Option<WorkConfig>,
    pub projects: Option<ProjectConfig>,
}

const CONFIG_FILE_NAME: &str = "reze.toml";
const DEFAULT_CONFIG_SUBDIRECTORY: &str = "reze";

pub fn default_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("failed to get path to home directory")?;
    let path = home_dir
        .join(".config")
        .join(DEFAULT_CONFIG_SUBDIRECTORY)
        .join(CONFIG_FILE_NAME);

    try_into_utf8pathbuf(&path)
}

fn try_into_utf8pathbuf(path: &std::path::Path) -> Result<PathBuf> {
    PathBuf::from_path_buf(path.to_path_buf()).map_err(|_| {
        anyhow!(
            "failed to parse path into UTF-8: {}",
            path.to_string_lossy()
        )
    })
}

fn find_in_ancestors(path: &Path, file_name: &str) -> Option<PathBuf> {
    for ancestor in path.ancestors() {
        if let Ok(entries) = ancestor.read_dir() {
            let target = entries
                .filter_map(|entry| entry.ok())
                .find(|entry| entry.path().is_file() && entry.file_name() == file_name)
                .and_then(|target| try_into_utf8pathbuf(&target.path()).ok());

            if target.is_some() {
                return target;
            }
        }
    }

    None
}

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

pub fn config_path() -> Option<PathBuf> {
    let cwd = env::current_dir()
        .context("failed to retrieve current working directory")
        .and_then(|path| try_into_utf8pathbuf(&path))
        .ok(); // TODO: logging?

    let ancestor_config_path = cwd.and_then(|dir| find_in_ancestors(&dir, CONFIG_FILE_NAME));
    ancestor_config_path.or_else(|| default_config_path().ok())
}

pub fn config() -> Option<Config> {
    config_path().and_then(|path| {
        File::open(path)
            .context("failed to open config")
            .and_then(|file| read_to_string(file).context("failed to read config"))
            .and_then(|content| {
                toml::from_str(&content).context("failed to parse config into TOML")
            })
            .ok() // TODO: logging?
    })
}
