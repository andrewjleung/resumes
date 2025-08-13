# resumes

> They say your resume needs side projects... why not make your resume the side project?

This repository acts as a VC for my resume contents using the [JSON Resume standard](https://jsonresume.org/), along with utilities for rendering my resume with [Typst](https://typst.app/).

View my latest resume [here](https://andrewjleung.github.io/resumes/AndrewLeung_Resume.pdf)!

## Installation

You will need the following prerequisites:

1. [Typst](https://github.com/typst/typst?tab=readme-ov-file#installation)
2. A [Rust](https://rustup.rs/) toolchain

Then, clone this repo and navigate to its root directory:

```sh
git clone https://github.com/andrewjleung/resumes.git
cd resumes # z if you have shiny-new-tool affliction
```

From here, you can compile and run the tool with `cargo run`.

You can view the CLI options with `cargo run -- --help`.

## Rendering

I use Typst to render my resume with a static template defined in [`template.typ`](./template.typ). On the side I use [`tdf`](https://github.com/itsjunetime/tdf) to view PDF renders in my terminal with hot reloading.

`template.typ` is set up to read in resume JSON from a given file path specified by the `data_path` key in the `sys.inputs` dictionary. This key can be set with the `--input` flag when compiling with Typst.

The CLI is then used to handle resume filtering, simple orchestration of `typst` compilation, and file system + artifact management:

1. Read and deserialize `resume.json`
2. Apply filters
3. Serialize back to JSON, write to an artifact
4. Compile `template.typ` to PDF, passing in a path to the filtered resume data
5. Read the raw PDF bytes from `typst`
6. Write the PDF

The main utility of this CLI is to separate resume data from resume view with declarative filtering. This way, you can keep record of all of your historical resume data within a resume JSON file while only rendering the notable or recent parts.

### Watch Mode

Running the CLI with the `-w` / `--watch` flag will run the CLI in watch mode.

In watch mode, the CLI process will stay alive until interrupted or killed and automatically re-render the resume when it detects changes to the resume JSON file's content.

This comes in handy if you have a PDF viewer with hot reloading like `tdf` on one side of your screen and an editor with resume content on the other side. 

## Publishing

Commits to `main` will trigger a [GitHub Actions workflow](./.github/workflows/static.yml) to render my resume as a PDF artifact and deploy it to GitHub Pages.

## Future work

- A terminal multiplexer preset to quickly open a live editing workspace
- Resume variants for tailored resumes
- Better logging
