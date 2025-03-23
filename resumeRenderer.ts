import { match } from 'ts-pattern';
import {
  BareResume,
  ResumeBasics,
  ResumeEducation,
  ResumeExperience,
  ResumeProject,
  ResumeSkill,
} from './types';

type RenderConfig = {
  font: string;
  fontSize: number;
  lineHeight: number;
  margin: number;
};

const DEFAULT_RENDER_CONFIG: RenderConfig = {
  font: 'carlito',
  fontSize: 11,
  margin: 1,
  lineHeight: 1,
};

const indent = (num: number): string => `\n${'  '.repeat(num)}`;

const renderResume = (
  config: Partial<RenderConfig>,
  sections: string[],
): string => {
  const configWithDefaults: RenderConfig = {
    ...DEFAULT_RENDER_CONFIG,
    ...config,
  };
  const marginDifference = configWithDefaults.margin - 1;

  return `\
%-------------------------
% Resume<T> in Latex
% Author : Andrew Leung
% Based off of: https://github.com/sb2nov/resume
% License : MIT
%------------------------

\\documentclass[letterpaper, ${configWithDefaults.fontSize}pt]{article}

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
\\usepackage[sfdefault]{${configWithDefaults.font}}

\\pagestyle{fancy}
\\fancyhf{} % clear all header and footer fields
\\fancyfoot{}
\\renewcommand{\\headrulewidth}{0pt}
\\renewcommand{\\footrulewidth}{0pt}

% Adjust margins
\\addtolength{\\oddsidemargin}{${marginDifference}in}
\\addtolength{\\evensidemargin}{${marginDifference}in}
\\addtolength{\\textwidth}{${-2 * marginDifference}in}
\\addtolength{\\topmargin}{${marginDifference}in}
\\addtolength{\\textheight}{${-2 * marginDifference}in}

\\urlstyle{same}

\\raggedbottom
\\raggedright
\\setlength{\\tabcolsep}{0in}

% Sections formatting
\\titleformat{\\section}{
  \\vspace{-4pt}\\scshape\\raggedright\\large
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
    \\begin{tabular*}{1\\textwidth}[t]{l@{\\extracolsep{\\fill}}r}
      \\small\\textbf{#1}\\space\\space\\small#2 | \\small#3 & \\small#4\\\\
    \\end{tabular*}\\vspace{-6pt}
}

\\newcommand{\\educationSubHeading}[5]{
  \\vspace{-2pt}\\item
    \\begin{tabular*}{1\\textwidth}[t]{l@{\\extracolsep{\\fill}}r}
      \\small\\textbf{#1}\\space\\space\\small#3, \\small#4 | \\small#5 & \\small#2
    \\end{tabular*}
}

\\newcommand{\\resumeJobSubheading}[3]{
  \\vspace{-2pt}\\item
    \\begin{tabular*}{1\\textwidth}[t]{l@{\\extracolsep{\\fill}}r}
      \\small\\textbf{#1}, \\small#3 & #2\\\\
    \\end{tabular*}
}

\\newcommand{\\resumeSubSubheading}[2]{
    \\item
    \\begin{tabular*}{1\\textwidth}{l@{\\extracolsep{\\fill}}r}
      \\small#1 & \\small #2\\\\
    \\end{tabular*}
}

\\newcommand{\\resumeProjectHeading}[3]{
    \\vspace{-2pt}\\item
    \\begin{tabular*}{1\\textwidth}{l@{\\extracolsep{\\fill}}r}
      \\small#1\\space\\space\\small#3 & \\small#2\\vspace{-1pt}\\\\
    \\end{tabular*}\\vspace{-5pt}
}

\\newcommand{\\resumeSubItem}[1]{\\resumeItem{#1}\\vspace{-4pt}}

\\renewcommand\\labelitemii{$\\vcenter{\\hbox{\\tiny$\\bullet$}}$}

\\newcommand{\\resumeSubHeadingListStart}{\\begin{itemize}[leftmargin=0in, label={}]}
\\newcommand{\\resumeSubHeadingListEnd}{\\end{itemize}}
\\newcommand{\\resumeItemListStart}{\\begin{itemize}[leftmargin=1.5em]}
\\newcommand{\\resumeItemListEnd}{\\end{itemize}\\vspace{-8pt}}

\\setstretch{${configWithDefaults.lineHeight}}

%-------------------------------------------
%%%%%%  RESUME STARTS HERE  %%%%%%%%%%%%%%%%%%%%%%%%%%%%

\\begin{document} 

${sections.join('')}

%-------------------------------------------
\\end{document}  
  `;
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
}: {
  basics: ResumeBasics;
}): string => {
  const profileUrls = profiles.map((profile) => profile.url);

  return `
\\begin{center}
  \\begin{minipage}[b]{0.5\\textwidth}
    \\raggedright
      \\textbf{\\Huge \\scshape ${name}} \\vspace{0pt} \\\\
      \\href{mailto:${email}}{${email}}
  \\end{minipage}%
  \\begin{minipage}[b]{0.5\\textwidth}
    \\raggedleft
      \\small \\href{${url}}{${url.slice('https://'.length)}}\\\\
      ${profileUrls.map((profileUrl) => `\\href{${profileUrl}}{${profileUrl.slice('https://'.length)}}`).join('\\\\')} \\vspace{0pt}
  \\end{minipage}%
\\end{center}
  `;
};

const renderMonthDate = (dateString: string): string => {
  if (dateString === 'Present') {
    return dateString;
  }

  const date = new Date(dateString);
  date.setDate(date.getDate() + 1);
  const month = date.toLocaleString('default', { month: 'short' });

  return `${month} ${date.getFullYear()}`;
};

const renderEducation = ({
  education,
  basics: {
    location: { city, region },
  },
}: { education: ResumeEducation[] } & { basics: ResumeBasics }): string => {
  if (education.length === 0) {
    return '';
  }

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
      {${studyType} ${area}}{${score} GPA}{${city}, ${region}}
  \\resumeSubHeadingListEnd
  `;
};

const renderSkills = ({ skills }: { skills: ResumeSkill[] }): string => {
  if (skills.length === 0) {
    return '';
  }

  const renderCategory = ({
    name,
    keywords,
  }: (typeof skills)[number]): string => {
    return `\\textbf{${name.replace('&', '\\&')}}{: ${keywords.join(
      ', ',
    )}}\\\\`;
  };

  // TODO: Figure out a sane way to work out proper composable indentation.
  const categories = skills.map(renderCategory).join(indent(6));

  return `
\\section{Skills}
    \\begin{itemize}[leftmargin=0in, label={}]
        \\small{\\item{
            ${categories}
        }}
    \\end{itemize}\\vspace{-16pt}
  `;
};

const renderExperience = ({
  name,
  location,
  position,
  startDate,
  endDate,
  highlights,
}: ResumeExperience): string => {
  const renderHighlight = (highlight: string) => `\\resumeItem{${highlight}}`;

  return `\\resumeSubheading{${name}}{${position}}{${location}}{${renderMonthDate(startDate)} -- ${renderMonthDate(endDate)}} 
            \\resumeItemListStart
                ${highlights.map(renderHighlight).join(indent(8))}
            \\resumeItemListEnd`;
};

const renderExperiences = ({ work }: { work: ResumeExperience[] }): string => {
  if (work.length < 1) {
    return '';
  }

  const experiences = work
    .sort((a, b) => Date.parse(b.startDate) - Date.parse(a.startDate))
    .map(renderExperience)
    .join(`\n${indent(4)}`);

  return `
\\section{Work Experience}
    \\resumeSubHeadingListStart    
        ${experiences}
    \\resumeSubHeadingListEnd
  `;
};

const renderProject = ({
  name,
  keywords,
  highlights,
  startDate,
  endDate,
  url,
}: ResumeProject): string => {
  const renderHighlight = (highlight: string): string =>
    `\\resumeItem{${highlight}}`;

  const renderedHighlights = highlights.map(renderHighlight).join(indent(8));

  const renderedUrl =
    url === undefined ? 'Closed Source' : url.slice('https://'.length);

  const dateRange =
    startDate === endDate
      ? startDate
      : endDate === undefined
        ? `${startDate} -- Present`
        : `${startDate} -- ${endDate}`;

  return `\\resumeProjectHeading
            {\\textbf{${name}}}{${dateRange}}{${renderedUrl}}
            \\resumeItemListStart
                ${renderedHighlights}
            \\resumeItemListEnd`;
};

const renderProjects = ({
  projects,
}: {
  projects: ResumeProject[];
}): string => {
  if (projects.length < 1) {
    return '';
  }

  return `
\\section{Projects}
    \\resumeSubHeadingListStart  
        ${projects.map(renderProject).join(indent(4))}
    \\resumeSubHeadingListEnd
  `;
};

const renderSection = (section: ResumeSection, resume: BareResume): string =>
  match(section)
    .returnType<string>()
    .with('education', () => renderEducation(resume))
    .with('experiences', () => renderExperiences(resume))
    .with('projects', () => renderProjects(resume))
    .with('skills', () => renderSkills(resume))
    .exhaustive();

type Filters = {
  include?: string[];
  exclude?: string[];
  after?: Date;
};

type ArbitrarilyNested =
  | string
  | string[]
  | {
      [key: string]: ArbitrarilyNested;
    };

type ResumeSection = 'skills' | 'experiences' | 'projects' | 'education';
type Filterable = { name: string; startDate: string };
type CheckError = { path: string; error: string };

class ResumeRenderer {
  private resume: BareResume;
  private shouldRunChecks: boolean = false;
  private experiencesConfig?: Filters;
  private projectsConfig?: Filters;

  constructor(resume: BareResume) {
    this.resume = { ...resume };
  }

  render(config: Partial<RenderConfig>, sections: ResumeSection[]): string {
    if (this.shouldRunChecks) this.runChecks();
    this.applyFilters();

    const body = sections.map((section) => renderSection(section, this.resume));

    return renderResume(config, [renderHeading(this.resume), ...body]);
  }

  experiences(ic: Filters): ResumeRenderer {
    this.experiencesConfig = ic;
    return this;
  }

  projects(ic: Filters): ResumeRenderer {
    this.projectsConfig = ic;
    return this;
  }

  withChecks() {
    this.shouldRunChecks = true;
    return this;
  }

  private findErrors(
    node: ArbitrarilyNested,
    errors: CheckError[],
    path: string[],
    cb: (text: string) => string | null,
  ) {
    if (typeof node === 'string') {
      const maybeError = cb(node);
      if (maybeError !== null)
        errors.push({ path: path.join('.'), error: maybeError });
    } else if (Array.isArray(node)) {
      node.forEach((v, i) => {
        path.push(i.toString());
        this.findErrors(v, errors, path, cb);
        path.pop();
      });
    } else {
      Object.entries(node).forEach(([key, value]) => {
        path.push(key);
        this.findErrors(value, errors, path, cb);
        path.pop();
      });
    }
  }

  private check(cb: (text: string) => string | null) {
    const errors: CheckError[] = [];
    const path: string[] = [];

    this.findErrors(
      this.resume as unknown as ArbitrarilyNested,
      errors,
      path,
      cb,
    );

    return errors;
  }

  private filter<T extends Filterable>(
    filterables: T[],
    filters: Filters,
  ): T[] {
    const { include, exclude, after } = filters;

    return filterables.filter(
      ({ name, startDate }) =>
        (include === undefined || include.some((n) => name.includes(n))) &&
        (exclude === undefined || !exclude.some((n) => name.includes(n))) &&
        (after === undefined ||
          (startDate !== undefined && after <= new Date(startDate))),
    );
  }

  private applyFilters() {
    if (this.resume.projects && this.projectsConfig) {
      this.resume.projects = this.filter(
        this.resume.projects,
        this.projectsConfig,
      );
    }

    if (this.resume.work && this.experiencesConfig) {
      this.resume.work = this.filter(this.resume.work, this.experiencesConfig);
    }
  }

  private runChecks() {
    const errors = this.check((text: string) => {
      if (text.match(/[^\\][\$\%]/)) {
        return 'Unescaped dollar sign ($)';
      }

      return null;
    });

    // TODO: Return warnings instead of logging them.
    if (errors.length > 0) {
      console.warn('\nWarnings:');
      console.warn(errors);
      console.warn();
    }
  }
}

export const resume = (resume: BareResume) => new ResumeRenderer(resume);
