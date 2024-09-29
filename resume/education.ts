import { ResumeSchema } from '@kurone-kito/jsonresume-types';

export default [
  {
    institution: 'Northeastern University',
    url: 'https://www.northeastern.edu/',
    area: 'Computer Science',
    studyType: 'B.S.',
    startDate: '2017-09-01',
    endDate: '2022-05-01',
    score: '3.9/4.0',
  },
] satisfies ResumeSchema['education'];
