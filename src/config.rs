pub mod resolution;

use schemars::JsonSchema;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::read_to_string;
use std::rc::Rc;

use serde::Deserialize;
use serde::Serialize;

use crate::config::resolution::config_path;
use crate::config::resolution::default_config_path;
use crate::prelude::*;
use crate::resume::query::Clause;

const DEFAULT_TITLE: &str = "resume";
const DEFAULT_TEMPLATE: &str = "template.typ";

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Config {
    #[schemars(with = "String")]
    pub template: Rc<Path>,

    #[serde(default)]
    pub queries: HashMap<String, Vec<Clause>>,

    pub title: Rc<String>,

    #[serde(default)]
    pub clean: bool,

    #[schemars(with = "String")]
    pub output_dir: Rc<Path>,

    #[schemars(with = "String")]
    pub resume_data_path: Rc<Path>,
}

impl Default for Config {
    fn default() -> Self {
        let current_dir = Rc::new(
            PathBuf::from_path_buf(env::current_dir().expect("current directory is accessible"))
                .expect("current directory is UTF-8"),
        );

        Config {
            template: Rc::clone(&current_dir).join(DEFAULT_TEMPLATE).into(),
            queries: HashMap::default(),
            title: Rc::new(String::from(DEFAULT_TITLE)),
            clean: false,
            output_dir: Rc::clone(&current_dir).as_path().into(),
            resume_data_path: Rc::clone(&current_dir).join("resume.json").into(),
        }
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
