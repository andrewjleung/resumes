use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
/// Utility for resume rendering and editing using the Resume JSON spec.
pub struct Args {
    /// The path to the Resume JSON file to render.
    #[arg(short, long)]
    pub resume: Option<String>,

    /// The directory to output rendering artifacts to.
    ///
    /// This directory will be recursively created.
    #[arg(short, long)]
    pub output_dir: Option<String>,

    /// A generic title used as the name for all generated artifacts.
    ///
    /// E.g. a title of "resume" will produce artifacts like:
    ///
    ///   - "resume.slice.json"
    ///   - "resume.pdf"
    #[arg(short, long)]
    pub title: Option<String>,

    /// If set, automatically remove all created intermediate artifacts, keeping the final render.
    #[arg(short, long)]
    pub clean: bool,

    /// If set, run the CLI in watch mode. This will automatically re-render the resume on changes
    /// to the resume JSON specified in the `resume` argument.
    /// TODO: Convert this into a subcommand
    #[arg(short, long)]
    pub watch: bool,

    /// Attempt to initialize a config file at '$HOME/.config/reze/reze.toml' if
    /// it does not exist already.
    #[arg(short, long)]
    pub init: bool,

    /// Path to the typst template to render with. This template should support receiving a path to
    /// a JSON file following the Resume JSON standard via system inputs like so:
    ///
    /// ```typst
    /// #template(json(sys.inputs.data_path))
    /// ```
    #[arg(long)]
    pub template: Option<String>,
}
