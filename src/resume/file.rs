use std::fs::read_to_string;

use crate::{prelude::*, resume::schema::Resume as MyResume};

pub fn read_toml(path: &Path) -> Result<MyResume> {
    let raw_resume = read_to_string(path)?;
    toml::from_str(&raw_resume)
        .map_err(Error::new)
        .context("failed to read resume json")
}
