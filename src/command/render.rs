use std::rc::Rc;

use crate::config::resolution::config_path;
use crate::prelude::*;
use figment::Figment;
use figment::providers::Format;

use crate::command::Run;
use crate::command::args::RenderArgs;
use crate::render::Render as RenderTrait;
use crate::typst::Typst;
use crate::{Config, resume};
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

        let mut resume = resume::file::read_toml(Rc::clone(&config.resume_data_path).as_ref())?;

        Typst()
            .render(&mut resume, &config)
            .context("failed to render resume with typst")?;

        Ok(())
    }
}

impl TryFrom<RenderArgs> for Config {
    type Error = Error;

    fn try_from(value: RenderArgs) -> Result<Self> {
        let mut figment = Figment::new().adjoin(figment::providers::Serialized::defaults(value));

        if let Some(path) = config_path() {
            figment = figment.adjoin(figment::providers::Toml::file(path));
        }

        let figment = figment.adjoin(Figment::from(RenderArgs::default()));

        figment.extract().map_err(anyhow::Error::new)
    }
}
