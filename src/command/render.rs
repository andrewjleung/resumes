use crate::command::Run;
use crate::render::Render as RenderTrait;
use crate::resume::ResumeSlice;
use crate::typst::Typst;
use crate::{prelude::*, resume};
use clap::Args;

#[derive(Args)]
pub struct Render {}

impl Run for Render {
    fn run(&self, config: &crate::config::Config) -> Result<()> {
        let resume = resume::file::read(&config.resume_data_path()?)?;
        let resume_slice = ResumeSlice::from_config(config, resume);

        Typst::new(config.typst.clone().unwrap_or_default())
            .render(resume_slice, config)
            .context("failed to render resume with typst")?;

        Ok(())
    }
}
