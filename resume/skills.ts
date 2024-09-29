import { ResumeSchema } from '@kurone-kito/jsonresume-types';

export default [
  {
    name: 'Programming Languages',
    level: 'Intermediate',
    keywords: ['TypeScript', 'JavaScript', 'Python', 'Ruby', 'SQL'],
  },
  {
    name: 'Tools & Frameworks',
    level: 'Intermediate',
    keywords: [
      'Git',
      'React',
      'Next.js',
      'Ruby on Rails',
      'HTML/CSS',
      'Tailwind',
      'PostgreSQL',
      'Vim',
    ],
  },
] satisfies ResumeSchema['skills'];
