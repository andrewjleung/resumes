use crate::{
    config::{
        Config, Title,
        typst::{Template, TypstConfig},
    },
    render::{Artifact, ArtifactKind, Render, Rendering},
    resume::schema::Resume,
};
use anyhow::{Context, Error, Result};
use std::{
    fs::{self, read},
    process::Command,
};

#[derive(Default)]
pub struct Typst();

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
            kind: ArtifactKind::Toml,
            content: resume_content.into_bytes(),
        };

        let title = match &config.title {
            Some(t) => t,
            None => &Title::default(),
        };

        resume_content_artifact
            .write(title, &output_dir)
            .context("failed to write resume slice")?;

        let Template(template) = match &config.typst {
            Some(TypstConfig { template: Some(t) }) => t,
            _ => &Template::default(),
        };

        let template_file_artifact = Artifact {
            kind: ArtifactKind::Typst,
            content: read(template)
                .context(format!("failed to read typst template: {template}"))?,
        };

        template_file_artifact.write(title, &output_dir)?;

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
                &format!("data_path={}", resume_content_artifact.file_name(title)),
                &template_file_artifact.file_name(title),
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
                kind: ArtifactKind::Pdf,
                content: output.stdout,
            },
        })
    }
}
