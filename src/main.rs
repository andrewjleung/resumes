use anyhow::{Context, Error, Result};
use camino::Utf8Path as Path;
use camino::Utf8PathBuf as PathBuf;
use chrono::NaiveDate;
use clap::Parser;
use json_resume::Resume;
use std::fs::DirBuilder;
use std::fs::read_to_string;

mod render;
mod resume;
mod typst;
mod view;
mod watcher;
mod world;

use render::Render;
use resume::{ResumeFilterPredicate::*, ResumeSlice};
use typst::Typst;
use world::World;

use crate::view::View;
use crate::watcher::watch;

static SECOND_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2020, 1, 6).unwrap();
static _THIRD_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2021, 1, 6).unwrap();

fn main_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.rs")
}

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

impl TryFrom<&Args> for World {
    type Error = anyhow::Error;

    fn try_from(args: &Args) -> Result<World> {
        DirBuilder::new()
            .recursive(true)
            .create(&args.output_dir)
            .map_err(Error::new)
            .context(format!(
                "failed to create output directory: {}",
                &args.output_dir
            ))?;

        let resume_data_path = Path::new(&args.resume);
        let main_source_path = main_path();

        Ok(World {
            artifact_title: args.title.clone(),
            clean: args.clean,
            output_dir: Path::new(&args.output_dir)
                .to_path_buf()
                .canonicalize_utf8()
                .context("failed to canonicalize output directory path")?,
            extra_watched_file_paths: vec![main_source_path],
            resume_data_path: resume_data_path.into(),
            watch: args.watch,
        })
    }
}

fn read_resume(path: &Path) -> Result<Resume> {
    let resume_json = read_to_string(path)?;
    serde_json::from_str(&resume_json)
        .map_err(Error::new)
        .context("failed to read resume json")
}

fn run_with_resume<F>(world: &World, f: F) -> Result<()>
where
    F: Fn(ResumeSlice) -> ResumeSlice,
{
    View::Updating.print(world)?;

    let resume = f(read_resume(&world.resume_data_path)?.into());
    Typst.render(resume, world)?;

    View::Updated.print(world)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let world = World::try_from(&args)?;

    let filter_resume = |resume: ResumeSlice| {
        resume
            .work([
                After(SECOND_COOP_START_DATE),
                Exclude(String::from("Sandbox at Northeastern University")),
            ])
            .projects([Include(String::from("Compiler for Python-like Language"))])
    };

    let result = if args.watch {
        watch(&world, || run_with_resume(&world, filter_resume))
    } else {
        run_with_resume(&world, filter_resume)
    };

    if let Err(e) = result {
        View::Error(e).print(&world)?
    };

    Ok(())
}
