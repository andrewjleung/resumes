import { ResumeSchema } from '@kurone-kito/jsonresume-types';
import basics from './basics';
import education from './education';
import skills from './skills';
import work from './work';
import projects from './projects';

export default {
  $schema:
    'https://raw.githubusercontent.com/jsonresume/resume-schema/v1.0.0/schema.json',
  basics,
  education,
  skills,
  work,
  projects,
  meta: {
    canonical:
      'https://raw.githubusercontent.com/jsonresume/resume-schema/master/resume.json',
    version: 'v1.0.0',
    lastModified: '2017-12-24T15:53:00',
  },
} satisfies ResumeSchema;
