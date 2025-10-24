use crate::{
    render::{Artifact, ArtifactKind, Render, Rendering},
    resume::ResumeSlice,
    world::World,
};
use anyhow::{Context, Error, Result};
use camino::Utf8Path as Path;
use regex::Regex;
use repo_path::repo_path;
use std::{fs::read, process::Command};

pub struct Typst;

impl Render for Typst {
    fn render_artifacts(&self, resume: ResumeSlice, config: &World) -> Result<Rendering> {
        let resume_json_content = serde_json::to_string(&resume.apply_slice())
            .context("failed to serialize resume data")?;

        // TODO: typst doesn't support parsing strings into datetimes, so this hack
        // will just find and replace all dates in the resume JSON with an object
        // containing a year, month, and date attribute
        let resume_json_content = Regex::new(r#""(\d{4})-0*([1-9][0-9]*|0)-0*([1-9][0-9]*|0)""#)
            .context("bad date regex")?
            .replace_all(
                &resume_json_content,
                "{\"year\":$1,\"month\":$2,\"day\":$3}",
            )
            .into_owned();

        let resume_json_artifact = Artifact {
            title: format!("{}.slice", config.artifact_title),
            kind: ArtifactKind::Json,
            content: resume_json_content.into_bytes(),
        };

        resume_json_artifact
            .write(&config.output_dir)
            .context("failed to write resume slice")?;

        // TODO: Make this an input instead of relying on location relative to repo
        let template_file_path = repo_path!("template.typ");
        let template_file_path = Path::from_path(template_file_path.as_path())
            .context("failed to create path to typst template")?;

        let template_file_artifact = Artifact {
            title: config.artifact_title.clone(),
            kind: ArtifactKind::Typst,
            content: read(template_file_path).context(format!(
                "failed to read typst template: {template_file_path}"
            ))?,
        };

        template_file_artifact.write(&config.output_dir)?;

        let output = Command::new("typst")
            .current_dir(&config.output_dir)
            .args([
                "compile",
                "-f",
                "pdf",
                "--input",
                &format!("data_path={}", resume_json_artifact.file_name()),
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
            intermediates: vec![resume_json_artifact, template_file_artifact],
            final_render: Artifact {
                title: config.artifact_title.clone(),
                kind: ArtifactKind::Pdf,
                content: output.stdout,
            },
        })
    }
}
