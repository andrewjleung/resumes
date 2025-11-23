use crate::{
    prelude::*,
    utils::path::{current_dir, normalize_path},
};

pub const CONFIG_FILE_NAME: &str = "reze.toml";
pub const DEFAULT_CONFIG_SUBDIRECTORY: &str = "reze";

pub fn default_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("failed to get path to home directory")?;
    let path = home_dir
        .join(".config")
        .join(DEFAULT_CONFIG_SUBDIRECTORY)
        .join(CONFIG_FILE_NAME);

    normalize_path(&path).context("failed to normalize default config path")
}

pub fn find_in_ancestors(path: &Path, file_name: &str) -> Option<PathBuf> {
    for ancestor in path.ancestors() {
        if let Ok(entries) = ancestor.read_dir() {
            let target = entries
                .filter_map(|entry| entry.ok())
                .find(|entry| entry.path().is_file() && entry.file_name() == file_name)
                .and_then(|target| normalize_path(&target.path()).ok());

            if target.is_some() {
                return target;
            }
        }
    }

    None
}

pub fn config_path() -> Option<PathBuf> {
    current_dir()
        .ok()
        .and_then(|dir| find_in_ancestors(&dir, CONFIG_FILE_NAME))
        .or_else(|| default_config_path().ok())
}
