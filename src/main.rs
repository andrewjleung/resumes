use anyhow::{Context, Error, Result};
use camino::Utf8Path as Path;
use chrono::NaiveDate;
use clap::Parser;
use json_resume::Resume;
use std::fs::DirBuilder;
use std::fs::read_to_string;

mod render;
mod resume;
mod typst;
mod watcher;

use render::Render;
use resume::{ResumeFilterPredicate::*, ResumeSlice};
use typst::Typst;

use crate::render::RenderConfig;
use crate::watcher::watch;

static _SECOND_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2020, 1, 6).unwrap();
static THIRD_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2021, 1, 6).unwrap();

// TODO: support STDIN
// TODO: filtering through CLI
// TODO: logging?

#[derive(Parser, Debug)]
#[command(version, about)]
/// Utility for resume rendering and editing using the Resume JSON spec.
struct Args {
    #[arg(short, long, default_value = "resume.json")]
    /// The path to the Resume JSON file to render.
    resume: String,

    #[arg(short, long, default_value = "artifacts")]
    /// The directory to output rendering artifacts to.
    ///
    /// This directory will be recursively created.
    output_dir: String,

    #[arg(short, long, default_value = "resume")]
    /// A generic title used as the name for all generated artifacts.
    ///
    /// E.g. a title of "resume" will produce artifacts like:
    ///
    ///   - "resume.slice.json"
    ///   - "resume.pdf"
    title: String,

    #[arg(short, long)]
    /// If set, automatically remove all created intermediate artifacts, keeping the final render.
    clean: bool,

    #[arg(short, long)]
    /// If set, run the CLI in watch mode. This will automatically re-render the resume on changes
    /// to the resume JSON specified in the `resume` argument.
    watch: bool,
}

impl TryFrom<&Args> for RenderConfig {
    type Error = anyhow::Error;

    fn try_from(args: &Args) -> Result<RenderConfig> {
        DirBuilder::new()
            .recursive(true)
            .create(&args.output_dir)
            .map_err(Error::new)
            .context(format!(
                "failed to create output directory: {}",
                &args.output_dir
            ))?;

        Ok(RenderConfig {
            clean: args.clean,
            output_dir: Path::new(&args.output_dir)
                .to_path_buf()
                .canonicalize_utf8()
                .context("failed to canonicalize output directory path")?,
            title: args.title.clone(),
        })
    }
}

fn read_resume(path: &Path) -> Result<Resume> {
    let resume_json = read_to_string(path)?;
    serde_json::from_str(&resume_json)
        .map_err(Error::new)
        .context("failed to read resume json")
}

fn run_with_resume<F>(args: &Args, f: F) -> Result<()>
where
    F: Fn(ResumeSlice) -> ResumeSlice,
{
    let resume = f(read_resume(Path::new(&args.resume))?.into());
    Typst.render(resume, &args.try_into()?)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let filter_resume = |resume: ResumeSlice| {
        resume
            .work([
                After(THIRD_COOP_START_DATE),
                Exclude(String::from("Sandbox at Northeastern University")),
            ])
            .projects([Include(String::from("Compiler for Python-like Language"))])
    };

    if args.watch {
        watch(Path::new(&args.resume), || {
            run_with_resume(&args, filter_resume)
        })
    } else {
        run_with_resume(&args, filter_resume)
    }
}
