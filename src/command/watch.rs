use crate::command::Run;
use crate::command::args::RenderArgs;
use crate::config::Config;
use crate::render::Render as RenderTrait;
use crate::typst::Typst;
use crate::view::View;
use crate::watcher::watch;
use crate::{prelude::*, resume};

use clap::Args;

#[derive(Args)]
pub struct Watch {
    #[command(flatten)]
    pub render_args: RenderArgs,
}

fn render(config: &Config) -> Result<()> {
    View::Updating.print(config, true).unwrap();
    let mut resume = resume::file::read_toml(&config.resume_data_path()?)?;

    Typst::new(config.typst.clone().unwrap_or_default())
        .render(&mut resume, config)
        .context("failed to render resume with typst")?;

    View::Updated.print(config, true).unwrap();
    Ok(())
}

impl Run for Watch {
    fn run(&self, config: &Config) -> Result<()> {
        watch(config.clone().into(), render)
    }
}
