use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Error, Result};
use chrono::NaiveDate;
use json_resume::Resume;

mod render;
mod resume;

use resume::{ResumeFilterPredicate::*, ResumeSlice};

static SECOND_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2020, 1, 6).unwrap();

fn read_resume() -> Result<Resume> {
    Err(Error::msg("Unimplemented!"))
}

fn main() {
    let document = ResumeSlice::from(read_resume().expect("failed to read resume data"))
        .work([
            After(SECOND_COOP_START_DATE),
            Exclude("Sandbox at Northeastern University".to_owned()),
        ])
        .projects([Include("Compiler for Python-like Language".to_owned())])
        .render_document()
        .expect("failed to render typst document");

    let mut child = Command::new("typst")
        .args(["compile", "-f pdf", "-", "-"])
        .spawn()
        .expect("failed to spawn typst process");

    let mut stdin = child.stdin.take().expect("failed to take ");
    let stdout = child.stdout.take().expect("failed to take ");

    stdin.write(document.as_bytes());

    // File::create(path)
    // stdout.bytes();

    // match rendered {
    //     Ok(_) => {
    //         println!("rendered!")
    //     }
    //     Err(e) => {
    //         eprintln!("{e:?}");
    //         exit(1);
    //     }
    // };
}
