use clap::Args;

#[derive(Args, Clone, Default)]
pub struct RenderArgs {
    /// If set, automatically remove all created intermediate artifacts, keeping the final render.
    #[arg(short, long)]
    pub clean: bool,

    /// The directory to output rendering artifacts to.
    ///
    /// This directory will be recursively created.
    #[arg(short, long)]
    pub output_dir: Option<String>,

    /// The path to the TOML resume file to render.
    #[arg(short, long)]
    pub resume: Option<String>,

    /// Path to the typst template to render with. This template should support receiving a path to
    /// a TOML resume.
    ///
    /// ```typst
    /// #template(toml(sys.inputs.data_path))
    /// ```
    #[arg(long)]
    pub template: Option<String>,

    /// A generic title used as the name for all generated artifacts.
    ///
    /// E.g. a title of "resume" will produce artifacts like:
    ///
    ///   - "resume.slice.json"
    ///   - "resume.pdf"
    #[arg(short, long)]
    pub title: Option<String>,
}
