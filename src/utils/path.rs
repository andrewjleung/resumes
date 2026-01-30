use std::fs;

use crate::prelude::*;

pub fn normalize_path(path: &std::path::Path) -> Result<PathBuf> {
    PathBuf::from_path_buf(path.to_path_buf())
        .map_err(|path| {
            anyhow!(
                "failed to coerce path into UTF-8 path: {}",
                path.to_string_lossy()
            )
        })?
        .canonicalize_utf8()
        .context(anyhow!(
            "failed to canonicalize path {}",
            path.to_string_lossy()
        ))
}

pub fn current_dir() -> Result<PathBuf> {
    std::env::current_dir()
        .map_err(Error::new)
        .and_then(|path| normalize_path(&path))
        .context("unable to access current working directory")
}

pub fn ensure_dir(dir: &Path) -> Result<()> {
    fs::DirBuilder::new()
        .recursive(true)
        .create(dir)
        .context(format!("failed to create directory at {}", dir))?;

    Ok(())
}
