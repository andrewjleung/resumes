use crate::command::{render::Render, watch::Watch};
use crate::config::typst::TypstConfig;
use crate::config::{Config, load};
use crate::prelude::*;
use clap::{Parser, Subcommand};
use merge::Merge;

pub mod render;
pub mod watch;

pub trait Run {
    fn run(&self, config: &Config) -> Result<()>;
}

#[derive(Parser)]
#[command(version, about)]
/// Utility for resume rendering and editing using the Resume JSON spec.
pub struct Reze {
    #[command(subcommand)]
    command: RezeCommand,

    /// If set, automatically remove all created intermediate artifacts, keeping the final render.
    #[arg(short, long)]
    pub clean: bool,

    /// The directory to output rendering artifacts to.
    ///
    /// This directory will be recursively created.
    #[arg(short, long)]
    pub output_dir: Option<String>,

    /// The path to the Resume JSON file to render.
    #[arg(short, long)]
    pub resume: Option<String>,

    /// Path to the typst template to render with. This template should support receiving a path to
    /// a JSON file following the Resume JSON standard via system inputs like so:
    ///
    /// ```typst
    /// #template(json(sys.inputs.data_path))
    /// ```
    #[arg(long)]
    pub template: Option<String>,

    /// A generic title used as the name for all generated artifacts.
    ///
    /// E.g. a title of "resume" will produce artifacts like:
    ///
    ///   - "resume.slice.json"
    ///   - "resume.pdf"
    #[arg(short, long)]
    pub title: Option<String>,
}

#[derive(Subcommand)]
pub enum RezeCommand {
    Render(Render),
    Watch(Watch),
}

impl Reze {
    fn config(&self) -> Result<Config> {
        let typst_config = TypstConfig::builder()
            .maybe_template(self.template.as_deref())?
            .build();

        let base_config = Config::builder()
            .maybe_artifact_title(self.title.clone())
            .clean(self.clean)
            .maybe_output_dir(self.output_dir.as_deref())?
            .maybe_resume_data_path(self.resume.as_deref())?
            .typst(typst_config)
            .build();

        Ok(base_config)
    }

    pub fn run(&self) -> Result<()> {
        let mut config = self.config().context("failed to create config from args")?;

        if let Some(file_config) = load() {
            config.merge(file_config);
        }

        match &self.command {
            RezeCommand::Render(cmd) => cmd.run(&config),
            RezeCommand::Watch(cmd) => cmd.run(&config),
        }
    }
}
