import resume from './resume';
import anonymousResume from './resume.anonymous';
import latex from 'node-latex';
import fs from 'fs';
import { resume as r, BareResume } from './resumeBuilder';

const LINE_HEIGHT = 1.1;
const SECOND_COOP_START_DATE = new Date('2020-01-06');

const renderAndWriteResume = (
  resume: BareResume,
  texFileName: string,
  pdfFileName: string,
): void => {
  const renderedResume = r(resume)
    .experiences({
      after: new Date(SECOND_COOP_START_DATE),
      exclude: ['Sandbox at Northeastern University'],
    })
    .projects({
      exclude: ['Random Audio Player', 'TheNeedleDrop Review Dataset'],
    })
    .render({ lineHeight: LINE_HEIGHT });

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

renderAndWriteResume(resume, 'main.tex', 'AndrewLeung_Resume.pdf');
renderAndWriteResume(anonymousResume, 'anonymous.tex', 'Anonymous_Resume.pdf');
