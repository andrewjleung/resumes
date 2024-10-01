import resume from './resume';
import latex from 'node-latex';
import fs from 'fs';
import { resume as r } from './resumeRenderer';
import { BareResume } from './types';

const LINE_HEIGHT = 1.1;
const SECOND_COOP_START_DATE = new Date('2020-01-06');

const ARTIFACTS_PATH = './artifacts';
const RESUME_TEX = 'main.tex';
const RESUME_PDF = 'AndrewLeung_Resume.pdf';

const artifact = (filename: string): string => `${ARTIFACTS_PATH}/${filename}`;

const writeResume = (
  renderedResume: string,
  texFileName: string,
  pdfFileName: string,
) => {
  // Write to an intermediate LaTeX file before generating PDF for debugging.
  fs.writeFileSync(texFileName, renderedResume, {
    encoding: 'utf-8',
    flag: 'w',
  });

  // Update PDF.
  const input = fs.createReadStream(texFileName);
  const output = fs.createWriteStream(pdfFileName);
  const pdf = latex(input);

  pdf.pipe(output);
  pdf.on('error', (err) => console.error(`${err}\n`));
  pdf.on('finish', () => console.log(`${pdfFileName} updated!`));
};

const renderAndWriteResume = (
  resume: BareResume,
  texFileName: string,
  pdfFileName: string,
): void => {
  const renderedResume = r(resume)
    .experiences({
      after: new Date(SECOND_COOP_START_DATE),
    })
    .projects({
      exclude: ['TheNeedleDrop Review Dataset', 'Random Audio Player'],
    })
    .render({ lineHeight: LINE_HEIGHT }, [
      'skills',
      'experiences',
      'projects',
      'education',
    ]);

  writeResume(renderedResume, texFileName, pdfFileName);
};

renderAndWriteResume(resume, artifact(RESUME_TEX), artifact(RESUME_PDF));
