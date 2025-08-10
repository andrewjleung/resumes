use anyhow::{Context, Result};
use std::{fs::File, io::Write, path::Path};

use crate::resume::ResumeSlice;

pub enum ArtifactKind {
    Json,
    Pdf,
}

impl ArtifactKind {
    fn extension(&self) -> String {
        match self {
            Self::Json => String::from("json"),
            Self::Pdf => String::from("pdf"),
        }
    }
}

pub struct Artifact {
    pub kind: ArtifactKind,
    pub content: Vec<u8>,
}

impl Artifact {
    pub fn write(&self, path: &Path, name: &str) -> Result<File> {
        let extension = self.kind.extension();
        let file_name = format!("{name}.{extension}");
        let mut file = File::create(path.join(file_name))
            .context(format!("failed to create {extension} file"))?;

        file.write(&self.content)
            .context(format!("failed to write to {extension} file"))?;

        Ok(file)
    }
}

#[allow(dead_code)]
pub struct Rendering {
    pub intermediates: Vec<Artifact>,
    pub final_render: Artifact,
}

pub trait Render {
    fn render(resume: ResumeSlice) -> Result<Rendering>;
}
