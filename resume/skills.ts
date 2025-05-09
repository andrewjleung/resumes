import { ResumeSchema } from '@kurone-kito/jsonresume-types';

export default [
  {
    name: 'Programming Languages',
    level: 'Intermediate',
    keywords: [
      'TypeScript',
      'JavaScript',
      'Python',
      'Ruby',
      'SQL',
      'HTML',
      'CSS',
    ],
  },
  {
    name: 'Tools & Frameworks',
    level: 'Intermediate',
    keywords: [
      'React',
      'Next.js',
      'Tailwind',
      'Ruby on Rails',
      'RSpec',
      'PostgreSQL',
      'Google BigQuery',
      'Git',
      'Neovim',
    ],
  },
] satisfies ResumeSchema['skills'];
