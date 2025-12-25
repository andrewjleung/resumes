mod command;
mod config;
mod prelude;
mod render;
mod resume;
mod typst;
mod utils;

use clap::Parser;
use prelude::*;

use crate::command::Reze;

// TODO: logging?

fn main() -> Result<()> {
    Reze::parse().run()
}
