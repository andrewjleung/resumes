use std::fs::read_to_string;

use json_resume::Resume;

use crate::prelude::*;

pub fn read(path: &Path) -> Result<Resume> {
    let resume_json = read_to_string(path)?;
    serde_json::from_str(&resume_json)
        .map_err(Error::new)
        .context("failed to read resume json")
}
