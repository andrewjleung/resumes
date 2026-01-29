use crate::command::render::Render;
use crate::command::schema::Schema;
use crate::config::resolution::config_path;
use crate::prelude::*;
use clap::{Parser, Subcommand};

mod args;
mod render;
mod schema;

pub trait Run {
    fn run(self) -> Result<()>;
}

#[derive(Parser)]
#[command(version, about)]
/// Utility for rendering resumes from TOML.
pub struct Reze {
    #[command(subcommand)]
    command: RezeCommand,
}

#[derive(Subcommand)]
enum RezeCommand {
    Config,
    Render(Render),
    Schema(Schema),
}

impl Reze {
    pub fn run_cli() -> Result<()> {
        Self::parse().run()
    }
}

impl Run for Reze {
    fn run(self) -> Result<()> {
        match self.command {
            RezeCommand::Render(cmd) => cmd.run(),
            RezeCommand::Config => {
                println!(
                    "{}",
                    config_path().ok_or(anyhow!("failed to get config path"))?
                );
                Ok(())
            }
            RezeCommand::Schema(cmd) => cmd.run(),
        }
    }
}
