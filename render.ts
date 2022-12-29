import resume from './resume.json';
import anonymousResume from './resume.anonymous.json';
import latex from 'node-latex';
import fs from 'fs';

type Resume = typeof resume;

const LINE_HEIGHT = 1.1;

const renderMonthDate = (dateString: string): string => {
  const date = new Date(dateString);
  date.setDate(date.getDate() + 1);
  const month = date.toLocaleString('default', { month: 'short' });

  return `${month} ${date.getFullYear()}`;
};

const renderResume = (...sections: string[]): string => {
  const begin = `
%-------------------------
% Resume in Latex
% Author : Andrew Leung
% Based off of: https://github.com/sb2nov/resume
% License : MIT
%------------------------

\\documentclass[letterpaper, 11pt]{article}

\\usepackage{latexsym}
\\usepackage[empty]{fullpage}
\\usepackage{titlesec}
\\usepackage{marvosym}
\\usepackage[usenames,dvipsnames]{color}
\\usepackage{verbatim}
\\usepackage{enumitem}
\\usepackage[hidelinks]{hyperref}
\\usepackage{fancyhdr}
\\usepackage[english]{babel}
\\usepackage{tabularx}
\\usepackage{setspace}
\\input{glyphtounicode}

%----------FONT OPTIONS----------
% sans-serif
% \\usepackage[sfdefault]{FiraSans}
% \\usepackage[sfdefault]{roboto}
% \\usepackage[sfdefault]{noto-sans}
% \\usepackage[default]{sourcesanspro}
\\usepackage[sfdefault]{carlito}

% serif
% \\usepackage{CormorantGaramond}
% \\usepackage{charter}


\\pagestyle{fancy}
\\fancyhf{} % clear all header and footer fields
\\fancyfoot{}
\\renewcommand{\\headrulewidth}{0pt}
\\renewcommand{\\footrulewidth}{0pt}

% Adjust margins
\\addtolength{\\oddsidemargin}{-0.5in}
\\addtolength{\\evensidemargin}{-0.5in}
\\addtolength{\\textwidth}{1in}
\\addtolength{\\topmargin}{-0.5in}
\\addtolength{\\textheight}{1.0in}

\\urlstyle{same}

\\raggedbottom
\\raggedright
\\setlength{\\tabcolsep}{0in}

% Sections formatting
\\titleformat{\\section}{
  \\vspace{-4pt}\\scshape\\raggedright\\normalsize
}{}{0em}{}[\\color{black}\\titlerule \\vspace{-5pt}]

% Ensure that generate pdf is machine readable/ATS parsable
\\pdfgentounicode=1

%-------------------------
% Custom commands
\\newcommand{\\resumeItem}[1]{
  \\item\\small{
    {#1 \\vspace{-2pt}}
  }
}

\\newcommand{\\resumeSubheading}[4]{
  \\vspace{-2pt}\\item
    \\begin{tabular*}{0.97\\textwidth}[t]{l@{\\extracolsep{\\fill}}r}
      \\small\\textbf{#1} & \\small#2\\vspace{-1pt}\\\\
      \\small#3 & \\small#4\\\\
    \\end{tabular*}\\vspace{-6pt}
}

\\newcommand{\\educationSubHeading}[5]{
  \\vspace{-2pt}\\item
    \\begin{tabular*}{0.97\\textwidth}[t]{l@{\\extracolsep{\\fill}}r}
      \\small\\textbf{#1} & \\small#2\\\\
      \\small#3, \\small#4 & \\small#5\\\\
    \\end{tabular*}
}

\\newcommand{\\resumeJobSubheading}[3]{
  \\vspace{-2pt}\\item
    \\begin{tabular*}{0.97\\textwidth}[t]{l@{\\extracolsep{\\fill}}r}
      \\small\\textbf{#1}, \\small#3 & #2\\\\
    \\end{tabular*}
}

\\newcommand{\\resumeSubSubheading}[2]{
    \\item
    \\begin{tabular*}{0.97\\textwidth}{l@{\\extracolsep{\\fill}}r}
      \\small#1 & \\small #2\\\\
    \\end{tabular*}
}

\\newcommand{\\resumeProjectHeading}[3]{
    \\item
    \\begin{tabular*}{0.97\\textwidth}{l@{\\extracolsep{\\fill}}r}
      \\small#1 & \\small#2\\vspace{-1pt}\\\\
      \\small{#3}
    \\end{tabular*}\\vspace{-6pt}
}

\\newcommand{\\resumeSubItem}[1]{\\resumeItem{#1}\\vspace{-4pt}}

\\renewcommand\\labelitemii{$\\vcenter{\\hbox{\\tiny$\\bullet$}}$}

\\newcommand{\\resumeSubHeadingListStart}{\\begin{itemize}[leftmargin=0.15in, label={}]}
\\newcommand{\\resumeSubHeadingListEnd}{\\end{itemize}}
\\newcommand{\\resumeItemListStart}{\\begin{itemize}}
\\newcommand{\\resumeItemListEnd}{\\end{itemize}\\vspace{-5pt}}

\\setstretch{${LINE_HEIGHT}}

%-------------------------------------------
%%%%%%  RESUME STARTS HERE  %%%%%%%%%%%%%%%%%%%%%%%%%%%%

\\begin{document} 
  `;

  const end = `
%-------------------------------------------
\\end{document}  
  `;

  return [begin, ...sections, end].join('');
};

const renderHeading = ({
  basics: {
    name,
    email,
    phone,
    url,
    profiles,
    location: { city, region },
  },
}: Resume): string => {
  const profileUrls = profiles.map((profile) => profile.url);
  const sites = profileUrls.map((site) => site.slice(8));

  return `
\\begin{center}
  \\begin{minipage}[b]{0.33333\\textwidth}
  \\raggedright
    \\small ${email}\\\\
    \\small ${phone}
  \\end{minipage}%
  \\begin{minipage}[b]{0.33333\\textwidth}
  \\centering
    \\textbf{\\huge \\scshape ${name}} \\\\ \\vspace{1.5pt}
  \\end{minipage}%
  \\begin{minipage}[b]{0.33333\\textwidth}
  \\raggedleft
    \\small ${sites.join('\\\\')}
  \\end{minipage}
\\end{center}
  `;
};

const renderEducation = ({
  education,
  basics: {
    location: { city, region },
  },
}: Resume): string => {
  const { institution, area, studyType, startDate, endDate, score } =
    education[0];

  // TODO: The resume.json standard doesn't include location for education...
  return `
\\section{Education}
  \\resumeSubHeadingListStart
    \\educationSubHeading
      {${institution}}{${renderMonthDate(startDate)} -- ${renderMonthDate(
    endDate,
  )}}
      {${studyType} ${area}}{GPA: ${score}}{${city}, ${region}}
      \\vspace{-6pt}
  \\resumeSubHeadingListEnd
  `;
};

const renderProgrammingSkills = ({ skills }: Resume): string => {
  const renderCategory = ({
    name,
    keywords,
  }: typeof skills[number]): string => {
    return `\\textbf{${name.replace('&', '\\&')}}{: ${keywords.join(
      ', ',
    )}}\\\\`;
  };

  // TODO: Figure out a sane way to work out proper composable indentation.
  const categories = skills.map(renderCategory).join('\n            ');

  return `
\\section{Skills}
    \\begin{itemize}[leftmargin=0.15in, label={}]
        \\small{\\item{
            ${categories}
        }}
        \\vspace{-2pt}
    \\end{itemize}
  `;
};

const renderExperience = ({
  name,
  location,
  position,
  startDate,
  endDate,
  highlights,
}: typeof resume.work[number]): string => {
  const renderHighlight = (highlight: string) => `\\resumeItem{${highlight}}`;

  return `\\resumeSubheading
            {${name}}{${renderMonthDate(startDate)} -- ${renderMonthDate(
    endDate,
  )}}{${position}}{${location}}
            \\resumeItemListStart
                ${highlights.map(renderHighlight).join('\n                ')}
            \\resumeItemListEnd`;
};

const renderExperiences = ({ work }: Resume): string => {
  const experiences = work.map(renderExperience).join('\n        ');

  return `
\\section{Work Experience}
    \\resumeSubHeadingListStart    
        ${experiences}
        \\vspace{-3pt}
    \\resumeSubHeadingListEnd
  `;
};

const renderProject = ({
  name,
  keywords,
  highlights,
  startDate,
  url,
}: typeof resume.projects[number]): string => {
  const renderHighlight = (highlight: string): string =>
    `\\resumeItem{${highlight}}`;

  const renderedKeywords = keywords.join(', ');
  const renderedHighlights = highlights
    .map(renderHighlight)
    .join('\n                ');

  const renderedUrl = url === undefined ? 'Closed Source' : url.slice(8);

  return `\\resumeProjectHeading
            {\\textbf{${name}} \\emph{${renderedKeywords}}}{${startDate}}{${renderedUrl}}
            \\resumeItemListStart
                ${renderedHighlights}
            \\resumeItemListEnd`;
};

const renderProjects = ({ projects }: Resume): string => {
  return `
\\section{Projects}
    \\resumeSubHeadingListStart  
        ${projects.map(renderProject).join('\n        ')}
    \\resumeSubHeadingListEnd
  `;
};

const renderAndWriteResume = (
  r: typeof resume,
  texFileName: string,
  pdfFileName: string,
): void => {
  // The ordering of arguments to `renderResume` correspond to the ordering of
  // sections on the resume.
  const renderedResume = renderResume(
    renderHeading(r),
    renderEducation(r),
    renderProgrammingSkills(r),
    renderExperiences(r),
    renderProjects(r),
  );

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
  pdf.on('error', (err) => console.error(err));
  pdf.on('finish', () => console.log(`${pdfFileName} updated!`));
};

renderAndWriteResume(resume, 'main.tex', 'AndrewLeung_Resume.pdf');
renderAndWriteResume(anonymousResume, 'anonymous.tex', 'Anonymous_Resume.pdf');
