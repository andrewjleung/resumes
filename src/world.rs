use camino::Utf8PathBuf as PathBuf;
use std::iter::once;

use crate::config::Config;

#[derive(Clone)]
pub struct World {
    pub artifact_title: String,
    pub clean: bool,
    pub config: Option<Config>,
    pub extra_watched_file_paths: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub resume_data_path: PathBuf,
    pub watch: bool,
}

impl World {
    pub fn watched_file_paths(&self) -> impl Iterator<Item = PathBuf> {
        once(self.resume_data_path.to_owned()).chain(self.extra_watched_file_paths.iter().cloned())
    }
}
