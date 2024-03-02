# resumes

This repository acts as a VC and source of truth for my resume using the [JSON Resume standard](https://jsonresume.org/).

It also contains a script `render.ts` for generating a PDF of my resume from JSON by rendering it in LaTeX, then converting it to PDF via `node-latex` (pdflatex under the hood).

In order to run this script, you'll first need to have LaTeX installed. You can install LaTeX [here](https://www.latex-project.org/get/). After this, install dependencies with `npm install`, then run `npx tsx render.ts` which will generate a PDF resume.

## VS Code Workflow

If you use VS Code, this project makes use of the [Run on Save extension by @emeraldwalk](https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave) to enable automatic rerendering of resume PDFs upon saving changes to any of the `.ts` files in the project and quick iteration.
