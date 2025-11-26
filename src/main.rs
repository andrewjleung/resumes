mod command;
mod config;
mod prelude;
mod render;
mod resume;
mod typst;
mod utils;
mod view;
mod watcher;

use clap::Parser;
use prelude::*;

use crate::command::Reze;

// TODO: support STDIN
// TODO: filtering through CLI
// TODO: logging?

fn main() -> Result<()> {
    Reze::parse().run()
}
