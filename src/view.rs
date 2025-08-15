use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    cursor::MoveToNextLine,
    execute,
    style::{Color, PrintStyledContent, Stylize},
    terminal::Clear,
};
use std::io::stdout;

use crate::world::World;

fn print_watching_prelude(world: &World) -> Result<()> {
    if !world.watch {
        return Ok(());
    }

    let watched_file_names: Vec<String> = world
        .watched_file_paths()
        .map(|path| path.file_name().unwrap_or(path.as_str()).to_owned())
        .collect();

    execute!(
        stdout(),
        Clear(crossterm::terminal::ClearType::All),
        MoveTo(0, 0),
        PrintStyledContent("watching ".with(Color::Cyan)),
        PrintStyledContent(watched_file_names.join(", ").with(Color::Reset)),
        MoveTo(0, 2),
    )?;

    Ok(())
}

pub enum View<'a> {
    Watching,
    Updating,
    Updated,
    Error(&'a anyhow::Error),
}

impl View<'_> {
    pub fn print(&self, world: &World) -> Result<()> {
        match self {
            View::Watching => print_watching_prelude(world),
            View::Updating => {
                print_watching_prelude(world)?;
                execute!(
                    stdout(),
                    PrintStyledContent("updating...".with(Color::Yellow)),
                )
                .map_err(Error::new)
            }
            View::Updated => {
                print_watching_prelude(world)?;
                execute!(
                    stdout(),
                    PrintStyledContent("content updated!".with(Color::Green)),
                )
                .map_err(Error::new)
            }
            View::Error(err) => {
                print_watching_prelude(world)?;
                execute!(
                    stdout(),
                    PrintStyledContent("⬤ ".with(Color::Red)),
                    PrintStyledContent(err.to_string().with(Color::Red)),
                    MoveToNextLine(1),
                    PrintStyledContent("└─ ".with(Color::Red)),
                    PrintStyledContent(err.root_cause().to_string().with(Color::Red)),
                )
                .map_err(Error::new)
            }
        }
        .context("failed to print to terminal")
    }
}
