use crate::{
    config::{Config, typst::TypstConfig},
    render::{Artifact, ArtifactKind, Render, Rendering},
    resume::schema::Resume,
};
use anyhow::{Context, Error, Result};
use std::{
    fs::{self, read},
    process::Command,
};

#[derive(Default)]
pub struct Typst {
    pub config: TypstConfig,
}

impl Typst {
    pub fn new(config: TypstConfig) -> Self {
        Typst { config }
    }
}

impl Render for Typst {
    fn render_artifacts(&self, resume: &Resume, config: &Config) -> Result<Rendering> {
        let output_dir = config.output_dir()?;

        fs::DirBuilder::new()
            .recursive(true)
            .create(&output_dir)
            .context(format!(
                "failed to create output directory at {}",
                output_dir
            ))?;

        let resume_content = toml::to_string(&resume).context("failed to serialize resume data")?;

        let resume_content_artifact = Artifact {
            title: format!("{}.slice", config.artifact_title()),
            kind: ArtifactKind::Toml,
            content: resume_content.into_bytes(),
        };

        resume_content_artifact
            .write(&output_dir)
            .context("failed to write resume slice")?;

        let template_file_path = self.config.template();
        let template_file_artifact = Artifact {
            title: config.artifact_title(),
            kind: ArtifactKind::Typst,
            content: read(&template_file_path).context(format!(
                "failed to read typst template: {template_file_path}"
            ))?,
        };

        template_file_artifact.write(&output_dir)?;

        let output = Command::new("typst")
            .current_dir(&output_dir)
            .args([
                "compile",
                "-f",
                "pdf",
                "--root",
                config
                    .resume_data_path()
                    .expect("a data path")
                    .parent()
                    .expect("a parent dir")
                    .as_str(),
                "--input",
                &format!("data_path={}", resume_content_artifact.file_name()),
                &template_file_artifact.file_name(),
                "-",
            ])
            .output()
            .context("failed to get compilation output from typst child process")?;

        if !output.status.success() {
            let exit_code = output.status.code();
            let stderr =
                String::from_utf8(output.stderr).context("typst failed, unable to read stderr")?;

            match exit_code {
                Some(code) => {
                    return Err(Error::msg(format!(
                        "typst failed with exit code {code}:\n\n{stderr}"
                    )));
                }
                None => return Err(Error::msg(format!("typst failed:\n\n{stderr}"))),
            }
        }

        Ok(Rendering {
            intermediates: vec![resume_content_artifact, template_file_artifact],
            final_render: Artifact {
                title: config.artifact_title(),
                kind: ArtifactKind::Pdf,
                content: output.stdout,
            },
        })
    }
}
