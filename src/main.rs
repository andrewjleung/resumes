use chrono::NaiveDate;
use clap::Parser;
use json_resume::Resume;
use std::fs::read_to_string;

mod command;
mod config;
mod prelude;
mod render;
mod resume;
mod typst;
mod utils;
mod view;
mod watcher;

use prelude::*;
use render::Render;
use resume::ResumeSlice;
use typst::Typst;

use crate::command::args::Args;
use crate::config::config::Config;
use crate::config::reload::ReloadableConfig;
use crate::view::View;
use crate::watcher::watch;

static _SECOND_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2020, 1, 6).unwrap();
static _THIRD_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2021, 1, 6).unwrap();

// TODO: support STDIN
// TODO: filtering through CLI
// TODO: logging?

fn read_resume(path: &Path) -> Result<Resume> {
    let resume_json = read_to_string(path)?;
    serde_json::from_str(&resume_json)
        .map_err(Error::new)
        .context("failed to read resume json")
}

fn rerender(config: &Config) -> Result<()> {
    View::Updating.print(config, true).unwrap();
    render(config)?;
    View::Updated.print(config, true).unwrap();
    Ok(())
}

fn render(config: &Config) -> Result<()> {
    let resume = read_resume(&config.resume_data_path()?)?;
    let resume_slice = ResumeSlice::from_config(config, resume);

    Typst {
        config: config.typst.clone().unwrap_or_default(),
    }
    .render(resume_slice, config)
    .context("failed to render resume with typst")?;

    Ok(())
}

fn run(args: &Args, config: ReloadableConfig) -> Result<()> {
    if args.watch {
        watch(config, rerender)
    } else {
        render(&config.current)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config: Config = args.clone().try_into()?;

    // TODO: Make this a subcommand
    if args.init {
        config::config::init().context("failed to initialize config")?;
    }

    run(&args, config.into())?;

    Ok(())
}
