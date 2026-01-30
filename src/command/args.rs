use clap::Args;
use figment::{
    Error, Metadata, Profile, Provider,
    value::{Dict, Map},
};
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Default, Serialize, Deserialize)]
pub struct RenderArgs {
    /// If set, automatically remove all created intermediate artifacts, keeping the final render.
    #[arg(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean: Option<bool>,

    /// The directory to output rendering artifacts to.
    ///
    /// This directory will be recursively created.
    #[arg(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dir: Option<String>,

    /// The path to the TOML resume file to render.
    #[arg(short, long)]
    #[serde(rename = "resume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_data_path: Option<String>,

    /// Path to the typst template to render with. This template should support receiving a path to
    /// a TOML resume.
    ///
    /// ```typst
    /// #template(toml(sys.inputs.data_path))
    /// ```
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// A generic title used as the name for all generated artifacts.
    ///
    /// E.g. a title of "resume" will produce artifacts like:
    ///
    ///   - "resume.slice.json"
    ///   - "resume.pdf"
    #[arg(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl Provider for RenderArgs {
    fn metadata(&self) -> Metadata {
        Metadata::named("Render Command Arguments")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        figment::providers::Serialized::defaults(Self::default()).data()
    }
}
