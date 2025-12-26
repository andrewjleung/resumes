use anyhow::{Context, Result};
use camino::Utf8Path as Path;
use std::fs::remove_file;
use std::{fs::File, io::Write};

use crate::config::{Config, Title};
use crate::resume::query::Query;
use crate::resume::schema::Resume;

#[allow(dead_code)]
pub enum ArtifactKind {
    Json,
    Toml,
    Pdf,
    Typst,
}

impl ArtifactKind {
    fn extension(&self) -> String {
        match self {
            Self::Json => String::from("json"),
            Self::Toml => String::from("toml"),
            Self::Pdf => String::from("pdf"),
            Self::Typst => String::from("typ"),
        }
    }
}

pub struct Artifact {
    pub kind: ArtifactKind,
    pub content: Vec<u8>,
}

impl Artifact {
    pub fn write(&self, title: &Title, dir: &Path) -> Result<File> {
        let file_name = self.file_name(title);
        let path = dir.join(&file_name);

        let mut file =
            File::create(&path).context(format!("failed to create file for artifact: {path}"))?;

        file.write(&self.content)
            .context(format!("failed to write to artifact file {path}"))?;

        Ok(file)
    }

    pub fn file_name(&self, title: &Title) -> String {
        let extension = self.kind.extension();
        format!("{}.{extension}", title)
    }
}

#[allow(dead_code)]
pub struct Rendering {
    pub intermediates: Vec<Artifact>,
    pub final_render: Artifact,
}

impl Rendering {
    pub fn clean(&self, dir: &Path, config: &Config) -> Result<()> {
        let title = match &config.title {
            Some(t) => t,
            None => &Title::default(),
        };

        for intermediate in &self.intermediates {
            let path = dir.join(intermediate.file_name(title));
            remove_file(&path).context(format!("failed to remove intermediate artifact: {path}"))?
        }

        Ok(())
    }
}

pub trait Render {
    fn render(&self, resume: &mut Resume, config: &Config) -> Result<Rendering> {
        // TODO: come up with a more flexible way to do this
        resume.experiences.retain_mut(|experience| {
            if experience.kind == "work"
                && let Some(wc) = &config.work
            {
                experience.query(&wc.queries)
            } else if experience.kind == "project"
                && let Some(pc) = &config.projects
            {
                experience.query(&pc.queries)
            } else {
                true
            }
        });

        let rendering = self.render_artifacts(resume, config)?;
        let output_dir = config.output_dir()?;
        let title = match &config.title {
            Some(t) => t,
            None => &Title::default(),
        };

        if config.clean {
            rendering.clean(&output_dir, config)?;
        }

        rendering
            .final_render
            .write(title, &output_dir)
            .context("failed to write final render")?;

        Ok(rendering)
    }

    fn render_artifacts(&self, resume: &Resume, config: &Config) -> Result<Rendering>;
}
