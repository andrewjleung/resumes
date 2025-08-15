use anyhow::Context;
use anyhow::Result;
use crossterm::cursor::MoveToPreviousLine;
use crossterm::queue;
use crossterm::{
    cursor::MoveTo,
    cursor::MoveToNextLine,
    style::{Color, PrintStyledContent, Stylize},
    terminal::Clear,
};
use std::io::Write;
use std::io::stdout;

use crate::world::World;

fn print_watching_prelude(world: &World) -> std::io::Result<()> {
    let watched_file_names: Vec<String> = world
        .watched_file_paths()
        .map(|path| path.file_name().unwrap_or(path.as_str()).to_owned())
        .collect();

    queue!(
        stdout(),
        Clear(crossterm::terminal::ClearType::All),
        MoveTo(0, 0),
        PrintStyledContent("watching ".to_string().with(Color::Cyan)),
        PrintStyledContent(watched_file_names.join(", ").with(Color::Reset)),
        MoveToNextLine(2)
    )
}

pub fn move_lines(lines: i32) -> std::io::Result<()> {
    if lines < 0 {
        queue!(
            stdout(),
            MoveToPreviousLine(lines.abs().try_into().unwrap())
        )
    } else {
        queue!(stdout(), MoveToNextLine(lines.try_into().unwrap()))
    }
}

pub enum View<'a> {
    Watching,
    Updating,
    Updated,
    Error(&'a anyhow::Error),
}

impl View<'_> {
    fn print_view(&self) -> std::io::Result<()> {
        match self {
            View::Watching => Ok(()),
            View::Updating => {
                queue!(
                    stdout(),
                    PrintStyledContent("updating...".with(Color::Yellow)),
                    MoveToNextLine(1)
                )
            }
            View::Updated => {
                queue!(
                    stdout(),
                    PrintStyledContent("content updated!".with(Color::Green)),
                    MoveToNextLine(1)
                )
            }
            View::Error(err) => {
                queue!(
                    stdout(),
                    PrintStyledContent("⬤ ".with(Color::Red)),
                    PrintStyledContent(err.to_string().with(Color::Red)),
                    MoveToNextLine(1),
                    PrintStyledContent("└ ".with(Color::Red)),
                    PrintStyledContent(err.root_cause().to_string().with(Color::Red)),
                    MoveToNextLine(1)
                )
            }
        }
    }

    pub fn print(&self, world: &World) -> Result<()> {
        if world.watch {
            print_watching_prelude(world)
                .and_then(|()| self.print_view())
                .and_then(|()| move_lines(1))
        } else {
            self.print_view()
        }
        .and_then(|()| stdout().flush())
        .context("failed to print to terminal")
    }
}
