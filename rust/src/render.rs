use anyhow::{Error, Result};
use bon::Builder;
use chrono::NaiveDate;
use json_resume::Resume;

use crate::{render::renderer_builder::State, resume::ResumeCopy};

pub const DEFAULT_FONT: &str = "carlito";
pub const DEFAULT_FONT_SIZE: u32 = 11;
pub const DEFAULT_LINE_HEIGHT: f32 = 1.0;
pub const DEFAULT_MARGIN: f32 = 1.0;

#[derive(Builder)]
pub struct Renderer {
    #[builder(default = DEFAULT_FONT.to_owned())]
    pub font: String,

    #[builder(default = DEFAULT_FONT_SIZE)]
    pub font_size: u32,

    #[builder(default = DEFAULT_LINE_HEIGHT)]
    pub line_height: f32,

    #[builder(default = DEFAULT_MARGIN)]
    pub margin: f32,
}

impl Renderer {
    pub fn render(&self, copy: &ResumeCopy) -> Result<String> {
        Err(Error::msg("Unimplemented!"))
    }
}
