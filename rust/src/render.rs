use std::fs::File;

use anyhow::{Context, Error, Result, anyhow};
use bon::Builder;

use crate::{render::render_builder::State, resume::RenderResumeSlice, resume::ResumeSlice};

pub const DEFAULT_FONT: &str = "carlito";
pub const DEFAULT_FONT_SIZE: u32 = 11;
pub const DEFAULT_LINE_HEIGHT: f32 = 1.0;
pub const DEFAULT_MARGIN: f32 = 1.0;

#[derive(Builder)]
pub struct Render {
    #[builder(default = DEFAULT_FONT.to_owned())]
    pub font: String,

    #[builder(default = DEFAULT_FONT_SIZE)]
    pub font_size: u32,

    #[builder(default = DEFAULT_LINE_HEIGHT)]
    pub line_height: f32,

    #[builder(default = DEFAULT_MARGIN)]
    pub margin: f32,
}

impl RenderResumeSlice for Render {
    fn render_resume_slice(
        resume: &json_resume::Resume,
        resume_slice: &ResumeSlice,
    ) -> Result<File> {
    }
}
