use anyhow::{Context, Result};
use camino::Utf8Path as Path;
use std::fs::remove_file;
use std::{fs::File, io::Write};

use crate::config::config::Config;
use crate::resume::ResumeSlice;

pub enum ArtifactKind {
    Json,
    Pdf,
    Typst,
}

impl ArtifactKind {
    fn extension(&self) -> String {
        match self {
            Self::Json => String::from("json"),
            Self::Pdf => String::from("pdf"),
            Self::Typst => String::from("typ"),
        }
    }
}

pub struct Artifact {
    pub title: String,
    pub kind: ArtifactKind,
    pub content: Vec<u8>,
}

impl Artifact {
    pub fn write(&self, dir: &Path) -> Result<File> {
        let file_name = self.file_name();
        let path = dir.join(&file_name);

        let mut file =
            File::create(&path).context(format!("failed to create file for artifact: {path}"))?;

        file.write(&self.content)
            .context(format!("failed to write to artifact file {path}"))?;

        Ok(file)
    }

    pub fn file_name(&self) -> String {
        let extension = self.kind.extension();
        format!("{}.{extension}", self.title)
    }
}

#[allow(dead_code)]
pub struct Rendering {
    pub intermediates: Vec<Artifact>,
    pub final_render: Artifact,
}

impl Rendering {
    pub fn clean(&self, dir: &Path) -> Result<()> {
        for intermediate in &self.intermediates {
            let path = dir.join(intermediate.file_name());
            remove_file(&path).context(format!("failed to remove intermediate artifact: {path}"))?
        }

        Ok(())
    }
}

pub trait Render {
    fn render(&self, resume: ResumeSlice, config: &Config) -> Result<Rendering> {
        let rendering = self.render_artifacts(resume, config)?;
        let output_dir = config.output_dir()?;

        if config.clean {
            rendering.clean(&output_dir)?;
        }

        rendering
            .final_render
            .write(&output_dir)
            .context("failed to write final render")?;

        Ok(rendering)
    }

    fn render_artifacts(&self, resume: ResumeSlice, config: &Config) -> Result<Rendering>;
}
