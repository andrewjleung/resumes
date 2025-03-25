import resume from './resume';
import latex from 'node-latex';
import fs from 'fs';
import { resume as r } from './resumeRenderer';
import { program } from 'commander';
import path from 'path';

const SECOND_COOP_START_DATE = new Date('2020-01-06');

const ARTIFACTS_PATH = './artifacts';
const RESUME_TEX = 'main.tex';
const RESUME_PDF = 'AndrewLeung_Resume.pdf';

const writeResumeTex = (renderedResume: string, texFileName: string) => {
  if (!fs.existsSync(path.dirname(texFileName))) {
    fs.mkdirSync(path.dirname(texFileName), { recursive: true });
  }

  fs.writeFileSync(texFileName, renderedResume, {
    encoding: 'utf-8',
    flag: 'w',
  });
};

const writeResume = (
  renderedResume: string,
  texFileName: string,
  pdfFileName: string,
) => {
  // Write to an intermediate LaTeX file before generating PDF for debugging.
  writeResumeTex(renderedResume, texFileName);

  // Update PDF.
  const input = fs.createReadStream(texFileName);
  const output = fs.createWriteStream(pdfFileName);
  const pdf = latex(input);

  pdf.pipe(output);
  pdf.on('error', (err) => console.error(`${err}\n`));
  pdf.on('finish', () => console.log(`${pdfFileName} updated!`));
};

program.option('-t, --tex-only', 'only create a .tex file');
program.option('-d, --dir <path>', 'output directory');
program.parse(process.argv);

const options = program.opts();
const renderedResume = r(resume)
  .experiences({
    after: new Date(SECOND_COOP_START_DATE),
    exclude: ['Sandbox at Northeastern University'],
  })
  .projects({
    include: ['Bookmark Manager', 'Compiler for Python-like Language'],
  })
  .render({ lineHeight: 1.2, margin: 0.75 }, [
    'skills',
    'experiences',
    'projects',
    'education',
  ]);

const outputDir = options.dir || ARTIFACTS_PATH;

if (options.texOnly) {
  writeResumeTex(renderedResume, `${outputDir}/${RESUME_TEX}`);
} else {
  writeResume(
    renderedResume,
    `${outputDir}/${RESUME_TEX}`,
    `${outputDir}/${RESUME_PDF}`,
  );
}
