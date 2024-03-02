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
  Pick<NonNullable<ResumeSchema['projects']>[number], 'url'>;

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
  \\begin{minipage}[b]{0.33333\\textwidth}
  \\raggedright
    \\small ${email}\\\\
    \\small ${phone}
  \\end{minipage}%
  \\begin{minipage}[b]{0.33333\\textwidth}
  \\centering
    \\textbf{\\huge \\scshape ${name}} \\\\ \\vspace{1.5pt}
    \\small ${personalUrl}
  \\end{minipage}%
  \\begin{minipage}[b]{0.33333\\textwidth}
  \\raggedleft
    \\small ${sites.join('\\\\')}
  \\end{minipage}
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
      {${studyType} ${area}}{GPA: ${score}}{${city}, ${region}}
      \\vspace{-6pt}
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
}: ResumeExperience): string => {
  const renderHighlight = (highlight: string) => `\\resumeItem{${highlight}}`;

  return `\\resumeSubheading
            {${name}}{${renderMonthDate(startDate)} -- ${renderMonthDate(
              endDate,
            )}}{${position}}{${location}}
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
}: ResumeProject): string => {
  const renderHighlight = (highlight: string): string =>
    `\\resumeItem{${highlight}}`;

  const renderedKeywords = keywords.join(', ');
  const renderedHighlights = highlights.map(renderHighlight).join(indent(8));

  const renderedUrl =
    url === undefined ? 'Closed Source' : url.slice('https://'.length);

  return `\\resumeProjectHeading
            {\\textbf{${name}} \\emph{${renderedKeywords}}}{${startDate}}{${renderedUrl}}
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

class Resume {
  private resume: BareResume;

  constructor(resume: BareResume) {
    this.resume = { ...resume };
  }

  experiences({ include, exclude, after }: InclusionConfig): Resume {
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

    return this;
  }

  projects({ include, exclude, after }: InclusionConfig): Resume {
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

    return this;
  }

  render(config: RenderConfig): string {
    return renderResume(config.lineHeight, [
      renderHeading(this.resume),
      renderEducation(this.resume),
      renderSkills(this.resume),
      renderExperiences(this.resume),
      renderProjects(this.resume),
    ]);
  }
}

export const resume = (resume: BareResume) => new Resume(resume);
