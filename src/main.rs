use anyhow::{Context, Error, Result};
use camino::Utf8Path as Path;
use camino::Utf8PathBuf as PathBuf;
use chrono::NaiveDate;
use clap::Parser;
use json_resume::Resume;
use std::fs::DirBuilder;
use std::fs::read_to_string;
use std::process::exit;
use std::vec;

mod config;
mod render;
mod resume;
mod typst;
mod view;
mod watcher;
mod world;

use render::Render;
use resume::ResumeSlice;
use typst::Typst;
use world::World;

use crate::config::config;
use crate::config::config_path;
use crate::view::View;
use crate::watcher::watch;

static _SECOND_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2020, 1, 6).unwrap();
static _THIRD_COOP_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(2021, 1, 6).unwrap();

fn main_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.rs")
}

fn template_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("template.typ")
}

// TODO: support STDIN
// TODO: filtering through CLI
// TODO: logging?

#[derive(Parser, Debug)]
#[command(version, about)]
/// Utility for resume rendering and editing using the Resume JSON spec.
struct Args {
    /// The path to the Resume JSON file to render.
    #[arg(short, long, default_value = "resume.json")]
    resume: String,

    /// The directory to output rendering artifacts to.
    ///
    /// This directory will be recursively created.
    #[arg(short, long, default_value = "artifacts")]
    output_dir: String,

    /// A generic title used as the name for all generated artifacts.
    ///
    /// E.g. a title of "resume" will produce artifacts like:
    ///
    ///   - "resume.slice.json"
    ///   - "resume.pdf"
    #[arg(short, long, default_value = "resume")]
    title: String,

    /// If set, automatically remove all created intermediate artifacts, keeping the final render.
    #[arg(short, long)]
    clean: bool,

    /// If set, run the CLI in watch mode. This will automatically re-render the resume on changes
    /// to the resume JSON specified in the `resume` argument.
    #[arg(short, long)]
    watch: bool,

    /// Attempt to initialize a config file at '$HOME/.config/reze/reze.toml' if
    /// it does not exist already.
    #[arg(short, long)]
    init: bool,
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
        let mut extra_watched_file_paths = vec![main_path(), template_path()];

        if let Some(path) = config_path() {
            extra_watched_file_paths.push(path);
        }

        Ok(World {
            artifact_title: args.title.clone(),
            clean: args.clean,
            output_dir: Path::new(&args.output_dir)
                .to_path_buf()
                .canonicalize_utf8()
                .context("failed to canonicalize output directory path")?,
            extra_watched_file_paths,
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
    View::Updating.print(world).unwrap();

    let resume = f(read_resume(&world.resume_data_path)?.into());
    Typst
        .render(resume, world)
        .context("failed to render resume with typst")?;

    View::Updated.print(world).unwrap();

    Ok(())
}

fn run(world: &mut World) -> Result<()> {
    let filter_resume = |resume: ResumeSlice| {
        let config = config().unwrap_or_default();

        let work_filters = config
            .work
            .and_then(|work| work.filters)
            .unwrap_or_default();

        let projects_filters = config
            .projects
            .and_then(|projects| projects.filters)
            .unwrap_or_default();

        resume.work(work_filters).projects(projects_filters)
    };

    if world.watch {
        watch(world, || run_with_resume(world, filter_resume))
    } else {
        run_with_resume(world, filter_resume)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.init {
        config::init().context("failed to initialize config")?;
    }

    let mut world = World::try_from(&args).context("failed to initialize")?;

    if let Err(e) = run(&mut world) {
        View::Error(&e).print(&world)?;
        exit(1);
    }

    Ok(())
}
