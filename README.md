# resumes

> They say your resume needs side projects... why not make your resume the side project?

This repository acts as a VC for my resume contents using a [custom TOML resume schema](./schemas/resume.schema.json), along with utilities rendering my resume with [Typst](https://typst.app/).

View my latest resume [here](https://andrewjleung.github.io/resumes/AndrewLeung_Resume.pdf)!

## Installation

You will need the following prerequisites:

1. [Typst](https://github.com/typst/typst?tab=readme-ov-file#installation)
2. A [Rust](https://rustup.rs/) toolchain

Then, clone this repo and navigate to its root directory. From here, you can compile and run the tool with `cargo run`.

You can view the CLI options with `cargo run -- --help`.

## Rendering

I use Typst to render my resume with a static template defined in [`template.typ`](./template.typ). On the side I use [`tdf`](https://github.com/itsjunetime/tdf) to view PDF renders in my terminal with hot reloading.

`template.typ` is set up to read in resume data from a given file path specified by the `data_path` key in the `sys.inputs` dictionary. This key can be set with the `--input` flag when compiling with Typst.

The CLI can then be used to handle resume "queries" (filters, overrides), simple orchestration of `typst` compilation, and file system + artifact management:

1. Read and deserialize resume data
2. Apply queries
3. Serialize back to TOML, write to an artifact
4. Render `template.typ` to PDF, passing in a path to the queried resume data
5. Read the raw PDF bytes from `typst`
6. Write the PDF

The main utility of this CLI is to separate resume data from resume view with declarative configuration for one or many variants. This way, you can keep record of all of your historical resume data within a resume TOML file while only rendering the notable, recent, or relevant parts.

## Configuration

A resume variant can be defined within a `reze.toml` file specifying:

- The variant's title
- Paths to the underlying data, output location, and Typst template
- Queries on the underlying data

A JSON schema definition for this configuration can be found in [config.schema.json](./schemas/config.schema.json).

## Publishing

Commits to `main` will trigger a [GitHub Actions workflow](./.github/workflows/static.yml) to render my resume as a PDF artifact and deploy it to GitHub Pages.

## Future work

- Better logging
