use crate::command::Run;
use crate::command::args::RenderArgs;
use crate::render::Render as RenderTrait;
use crate::typst::Typst;
use crate::{prelude::*, resume};
use clap::Args;

#[derive(Args)]
pub struct Render {
    #[command(flatten)]
    pub render_args: RenderArgs,
}

impl Run for Render {
    fn run(&self, config: &crate::config::Config) -> Result<()> {
        let mut resume = resume::file::read_toml(&config.resume_data_path()?)?;

        Typst()
            .render(&mut resume, config)
            .context("failed to render resume with typst")?;

        Ok(())
    }
}
