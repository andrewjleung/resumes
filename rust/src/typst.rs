use std::{path::Path, process::Command};

use crate::{
    render::{Artifact, ArtifactKind, Render, Rendering},
    resume::ResumeSlice,
};
use anyhow::{Context, Error, Result};
use regex::Regex;
use repo_path_lib::repo_dir;

const TEMPLATE_FILE_NAME: &str = "template.typ";

pub struct Typst;

impl Render for Typst {
    fn render(resume: ResumeSlice) -> Result<Rendering> {
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
            kind: ArtifactKind::Json,
            content: resume_json_content.into_bytes(),
        };

        resume_json_artifact
            .write(Path::new("./artifacts"), "resume_slice")
            .context("failed to write resume slice")?;

        let output = Command::new("typst")
            .current_dir(repo_dir().join("rust"))
            .args(["compile", "-f", "pdf", TEMPLATE_FILE_NAME, "-"])
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
            intermediates: vec![resume_json_artifact],
            final_render: Artifact {
                kind: ArtifactKind::Pdf,
                content: output.stdout,
            },
        })
    }
}
