use anyhow::{Context, Result};
use camino::Utf8Path as Path;
use std::fs::remove_file;
use std::rc::Rc;
use std::{fs::File, io::Write};

use crate::config::Config;
use crate::resume::schema::Resume;

#[allow(dead_code)]
pub enum ArtifactKind {
    Json,
    Toml,
    Pdf,
    Typst,
}

pub struct Artifact {
    pub path: Rc<Path>,
    pub clean: bool,
}

impl Artifact {
    pub fn write(path: Rc<Path>, bytes: &[u8], clean: bool) -> Result<Self> {
        let mut file = File::create(path.as_ref())
            .context(format!("failed to create file for artifact: {path}"))?;

        file.write(bytes)
            .context(format!("failed to write to artifact file {path}"))?;

        Ok(Self {
            path: Rc::clone(&path),
            clean,
        })
    }

    // pub fn write(&self, dir: &Path) -> Result<File> {
    //     let file_name = self.file_name(&self.title);
    //     let path = dir.join(&file_name);
    //
    //     let mut file =
    //         File::create(&path).context(format!("failed to create file for artifact: {path}"))?;
    //
    //     file.write(&self.content)
    //         .context(format!("failed to write to artifact file {path}"))?;
    //
    //     Ok(file)
    // }
    //
    // pub fn file_name(&self, title: &Title) -> String {
    //     let extension = self.kind.extension();
    //     format!("{}.{extension}", title)
    // }
}

impl Drop for Artifact {
    fn drop(&mut self) {
        if !self.clean {
            return;
        }

        // TODO: log if this fails
        let _ = remove_file(self.path.as_ref());
    }
}

#[allow(dead_code)]
pub struct Rendering {
    pub intermediates: Vec<Artifact>,
    pub final_render: Artifact,
}

pub trait Render {
    fn render(&self, resume: &mut Resume, config: &Config) -> Result<Rendering> {
        for (kind, clauses) in config.queries.iter() {
            resume.query_experiences_by_kind(kind, clauses);
        }

        let rendering = self.render_artifacts(resume, config)?;

        Ok(rendering)
    }

    fn render_artifacts(&self, resume: &Resume, config: &Config) -> Result<Rendering>;
}
