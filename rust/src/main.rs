use std::process::exit;

use anyhow::{Error, Result};
use chrono::NaiveDate;
use json_resume::Resume;

mod render;
mod resume;

use render::Renderer;

use resume::ResumeCopy;
use resume::ResumeFilterPredicate::*;
use resume::ResumeSection::*;

const fn const_unwrap<T: Copy>(x: Option<T>) -> T {
    if let Some(x) = x { x } else { panic!("") }
}

const SECOND_COOP_START_DATE: NaiveDate = const_unwrap(NaiveDate::from_ymd_opt(2020, 1, 6));

fn read_resume() -> Result<Resume> {
    Err(Error::msg("Unimplemented!"))
}

fn main() {
    let resume_data = read_resume().expect("failed to access resume");
    let resume_copy = ResumeCopy::builder()
        .resume(resume_data)
        .work([
            After(SECOND_COOP_START_DATE),
            Exclude("Sandbox at Northeastern University".to_owned()),
        ])
        .projects([Include("Compiler for Python-like Language".to_owned())])
        .sections([Skills, Experiences, Projects, Education])
        .build();

    let rendered = Renderer::builder()
        .line_height(1.25)
        .build()
        .render(&resume_copy);

    match rendered {
        Ok(rendering) => {
            println!("{rendering}");
        }
        Err(e) => {
            eprintln!("{e:?}");
            exit(1);
        }
    };
}
