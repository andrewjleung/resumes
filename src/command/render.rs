use crate::command::Run;
use crate::command::args::RenderArgs;
use crate::config::Title;
use crate::render::Render as RenderTrait;
use crate::typst::Typst;
use crate::{Config, TypstConfig, prelude::*, resume};
use clap::Args;

#[derive(Args)]
pub struct Render {
    #[command(flatten)]
    pub render_args: RenderArgs,
}

impl Run for Render {
    fn run(self) -> Result<()> {
        let config: Config = self
            .render_args
            .try_into()
            .context("failed to initialize config for render command")?;

        let mut resume = resume::file::read_toml(&config.resume_data_path()?)?;

        Typst()
            .render(&mut resume, &config)
            .context("failed to render resume with typst")?;

        Ok(())
    }
}

impl TryFrom<RenderArgs> for Config {
    type Error = Error;

    fn try_from(value: RenderArgs) -> Result<Self> {
        let typst_config = TypstConfig::builder()
            .maybe_template(value.template.as_deref())?
            .build();

        let config = Config::builder()
            .maybe_title(value.title.map(Title))
            .clean(value.clean)
            .maybe_output_dir(value.output_dir.as_deref())?
            .maybe_resume_data_path(value.resume.as_deref())?
            .typst(typst_config)
            .build();

        config.merge_with_file_config()
    }
}
