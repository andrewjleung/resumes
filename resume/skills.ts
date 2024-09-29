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
      'Git',
      'React',
      'Next.js',
      'Ruby on Rails',
      'Tailwind',
      'PostgreSQL',
      'BigQuery',
      'Vim',
    ],
  },
] satisfies ResumeSchema['skills'];
