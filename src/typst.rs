use crate::prelude::*;
use crate::{
    config::Config,
    render::{Artifact, Render, Rendering},
    resume::schema::Resume,
    utils::path::ensure_dir,
};
use std::rc::Rc;
use std::{fs::read, process::Command};

#[derive(Default)]
pub struct Typst();

fn resume_content(resume: &Resume) -> Result<Vec<u8>> {
    Ok(toml::to_string(&resume)
        .context("failed to serialize resume data")?
        .into_bytes())
}

fn template_content(config: &Config) -> Result<Vec<u8>> {
    read(Rc::clone(&config.template).as_ref()).context(format!(
        "failed to read typst template: {}",
        config.template
    ))
}

impl Render for Typst {
    fn render_artifacts(&self, resume: &Resume, config: &Config) -> Result<Rendering> {
        ensure_dir(&config.output_dir)?;

        let resume_content_filename = format!("{}.{}", config.title, "toml");
        let resume_content_artifact = Artifact::write(
            Rc::clone(&config.output_dir)
                .join(Path::new(&resume_content_filename))
                .into(),
            &resume_content(resume)?,
            config.clean,
        )
        .context("failed to write resume content artifact")?;

        let template_content_filename = format!("{}.{}", config.title, "typ");
        let template_content_artifact = Artifact::write(
            Rc::clone(&config.output_dir)
                .join(Path::new(&template_content_filename))
                .into(),
            &template_content(config)?,
            config.clean,
        )
        .context("failed to write template content artifact")?;

        let output = Command::new("typst")
            .current_dir(config.output_dir.as_ref())
            .args([
                "compile",
                "-f",
                "pdf",
                "--root",
                ".",
                "--input",
                &format!("data_path={}", resume_content_filename),
                &template_content_filename,
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

        let final_render_filename = format!("{}.{}", config.title, "pdf");
        let final_render_artifact = Artifact::write(
            Rc::clone(&config.output_dir)
                .join(Path::new(&final_render_filename))
                .into(),
            &output.stdout,
            false,
        )
        .context("failed to write final rendering artifact")?;

        Ok(Rendering {
            intermediates: vec![resume_content_artifact, template_content_artifact],
            final_render: final_render_artifact,
        })
    }
}
