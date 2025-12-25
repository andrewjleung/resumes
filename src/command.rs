use crate::command::render::Render;
use crate::command::schema::Schema;
use crate::config::resolution::config_path;
use crate::config::typst::TypstConfig;
use crate::config::{Config, Title, load};
use crate::prelude::*;
use clap::{Parser, Subcommand};
use merge::Merge;

pub mod args;
pub mod render;
pub mod schema;

pub trait Run {
    fn run(&self, config: &Config) -> Result<()>;
}

#[derive(Parser)]
#[command(version, about)]
/// Utility for rendering resumes from TOML.
pub struct Reze {
    #[command(subcommand)]
    command: RezeCommand,
}

#[derive(Subcommand)]
pub enum RezeCommand {
    Config,
    Render(Render),
    Schema(Schema),
}

impl Reze {
    fn config(&self) -> Result<Config> {
        let args = match &self.command {
            RezeCommand::Render(cmd) => cmd.render_args.clone(),
            _ => return load(),
        };

        let typst_config = TypstConfig::builder()
            .maybe_template(args.template.as_deref())?
            .build();

        let base_config = Config::builder()
            .maybe_title(args.title.map(Title))
            .clean(args.clean)
            .maybe_output_dir(args.output_dir.as_deref())?
            .maybe_resume_data_path(args.resume.as_deref())?
            .typst(typst_config)
            .build();

        Ok(base_config)
    }

    pub fn run(&self) -> Result<()> {
        let mut config = self.config().context("failed to create config from args")?;

        if let Ok(file_config) = load() {
            config.merge(file_config);
        }

        match &self.command {
            RezeCommand::Render(cmd) => cmd.run(&config),
            RezeCommand::Config => {
                println!(
                    "{}",
                    config_path().ok_or(anyhow!("failed to get config path"))?
                );
                Ok(())
            }
            RezeCommand::Schema(cmd) => cmd.run(&config),
        }
    }
}
