use std::{
    fs::{DirBuilder, read_to_string},
    path::Path,
};

use anyhow::{Context, Error, Result};
use chrono::NaiveDate;
use json_resume::Resume;

mod render;
mod resume;
mod typst;

use render::Render;
use repo_path_lib::repo_dir;
use resume::{ResumeFilterPredicate::*, ResumeSlice};
use typst::Typst;

static SECOND_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2020, 1, 6).unwrap();

fn read_resume(path: &Path) -> Result<Resume> {
    let resume_json = read_to_string(path)?;
    serde_json::from_str(&resume_json)
        .map_err(Error::new)
        .context("failed to read resume json")
}

fn main() {
    DirBuilder::new()
        .recursive(true)
        .create(repo_dir().join("rust/artifacts"))
        .map_err(Error::new)
        .context("failed to create artifacts directory")
        .unwrap();

    let resume = read_resume(&repo_dir().join("rust/resume.json")).unwrap();
    let resume = ResumeSlice::from(resume)
        .work([
            After(SECOND_COOP_START_DATE),
            Exclude(String::from("Sandbox at Northeastern University")),
        ])
        .projects([Include(String::from("Compiler for Python-like Language"))]);

    let pdf_content = Typst::render(resume).unwrap();

    pdf_content
        .final_render
        .write(&repo_dir().join("rust/artifacts"), "resume")
        .unwrap();

    println!("success!")
}
