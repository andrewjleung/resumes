import { ResumeSchema } from '@kurone-kito/jsonresume-types';

export type RenderConfig = {
  lineHeight: number;
};

type ResumeBasics = Required<
  Pick<NonNullable<ResumeSchema['basics']>, 'name' | 'email' | 'phone' | 'url'>
> & {
  location: Required<
    Pick<
      NonNullable<NonNullable<ResumeSchema['basics']>['location']>,
      'city' | 'region'
    >
  >;
  profiles: Required<
    Pick<
      NonNullable<
        NonNullable<NonNullable<ResumeSchema['basics']>['profiles']>
      >[number],
      'url'
    >
  >[];
};

type ResumeEducation = Required<
  Pick<
    NonNullable<ResumeSchema['education']>[number],
    'institution' | 'area' | 'studyType' | 'startDate' | 'endDate' | 'score'
  >
>;

type ResumeSkill = Required<
  Pick<NonNullable<ResumeSchema['skills']>[number], 'name' | 'keywords'>
>;

type ResumeExperience = Required<
  Pick<
    NonNullable<ResumeSchema['work']>[number],
    'name' | 'location' | 'position' | 'startDate' | 'endDate' | 'highlights'
  >
>;

type ResumeProject = Required<
  Pick<
    NonNullable<ResumeSchema['projects']>[number],
    'name' | 'keywords' | 'highlights' | 'startDate'
  >
> &
  Pick<NonNullable<ResumeSchema['projects']>[number], 'url' | 'endDate'>;

export type BareResume = {
  basics: ResumeBasics;
  education: ResumeEducation[];
  skills: ResumeSkill[];
  work: ResumeExperience[];
  projects: ResumeProject[];
};

type _BareResumeCheck = BareResume extends ResumeSchema ? BareResume : never;

const indent = (num: number): string => `\n${'  '.repeat(num)}`;

const renderResume = (lineHeight: number, sections: string[]): string =>
  `\
%-------------------------
% Resume<T> in Latex
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
\\newcommand{\\resumeItemListEnd}{\\end{itemize}\\vspace{-5pt}}

\\setstretch{${lineHeight}}

%-------------------------------------------
%%%%%%  RESUME STARTS HERE  %%%%%%%%%%%%%%%%%%%%%%%%%%%%

\\begin{document} 

${sections.join('')}

%-------------------------------------------
\\end{document}  
  `;

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
  const sites = profileUrls.map((site) => site.slice('https://'.length));
  const personalUrl = url.slice('https://'.length);

  return `
\\begin{center}
  \\begin{minipage}[b]{0.5\\textwidth}
    \\raggedright
      \\textbf{\\Huge \\scshape ${name}} \\vspace{0pt} \\\\
      ${email}
  \\end{minipage}%
  \\begin{minipage}[b]{0.5\\textwidth}
    \\raggedleft
      \\small ${personalUrl}\\\\${sites.join('\\\\')} \\vspace{0pt}
  \\end{minipage}%
\\end{center}
  `;
};

const renderMonthDate = (dateString: string): string => {
  if (dateString === 'Present') {
    return 'Present';
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
    .join('\n        ');

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

type InclusionConfig = {
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

class Resume {
  private resume: BareResume;
  private doPrecheck: boolean = false;
  private experiencesConfig?: InclusionConfig;
  private projectsConfig?: InclusionConfig;

  constructor(resume: BareResume) {
    this.resume = { ...resume };
  }

  experiences(ic: InclusionConfig): Resume {
    this.experiencesConfig = ic;
    return this;
  }

  private applyExperiencesFilters() {
    if (this.experiencesConfig === undefined) return;

    const { include, exclude, after } = this.experiencesConfig;

    if (include !== undefined) {
      this.resume.work = this.resume.work?.filter(
        ({ name }) =>
          name !== undefined && include.some((n) => name.includes(n)),
      );
    }

    if (exclude !== undefined) {
      this.resume.work = this.resume.work?.filter(
        ({ name }) =>
          name !== undefined && !exclude.some((n) => name.includes(n)),
      );
    }

    if (after !== undefined) {
      this.resume.work = this.resume.work?.filter(({ startDate }) => {
        return startDate !== undefined && after <= new Date(startDate);
      });
    }
  }

  projects(ic: InclusionConfig): Resume {
    this.projectsConfig = ic;
    return this;
  }

  private applyProjectsFilters() {
    if (this.projectsConfig === undefined) return;

    const { include, exclude, after } = this.projectsConfig;

    if (include !== undefined) {
      this.resume.projects = this.resume.projects?.filter(
        ({ name }) =>
          name !== undefined && include.some((n) => name.includes(n)),
      );
    }

    if (exclude !== undefined) {
      this.resume.projects = this.resume.projects?.filter(
        ({ name }) =>
          name !== undefined && !exclude.some((n) => name.includes(n)),
      );
    }

    if (after !== undefined) {
      this.resume.projects = this.resume.projects?.filter(({ startDate }) => {
        return startDate !== undefined && after <= new Date(startDate);
      });
    }
  }

  private check(
    node: ArbitrarilyNested,
    errors: [string, string][],
    path: string[],
    cb: (text: string) => string | null,
  ) {
    if (typeof node === 'string') {
      const maybeError = cb(node);
      if (maybeError !== null) errors.push([path.join('.'), maybeError]);
    } else if (Array.isArray(node)) {
      node.forEach((v, i) => {
        path.push(i.toString());
        this.check(v, errors, path, cb);
        path.pop();
      });
    } else {
      Object.entries(node).forEach(([key, value]) => {
        path.push(key);
        this.check(value, errors, path, cb);
        path.pop();
      });
    }
  }

  private precheck(cb: (text: string) => string | null) {
    const errors: [string, string][] = [];
    const path: string[] = [];

    this.check(this.resume as unknown as ArbitrarilyNested, errors, path, cb);

    return errors;
  }

  withPrecheck() {
    this.doPrecheck = true;
    return this;
  }

  render(config: RenderConfig): string {
    if (this.doPrecheck) {
      const errors = this.precheck((text: string) => {
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

    this.applyExperiencesFilters();
    this.applyProjectsFilters();

    return renderResume(config.lineHeight, [
      renderHeading(this.resume),
      renderExperiences(this.resume),
      renderProjects(this.resume),
      renderSkills(this.resume),
      renderEducation(this.resume),
    ]);
  }
}

export const resume = (resume: BareResume) => new Resume(resume);
