import { ResumeSchema } from '@kurone-kito/jsonresume-types';

export default [
  {
    name: 'PayPal',
    location: 'Remote',
    description: 'Braintree post-processing, treasury funding.',
    position: 'Software Engineer 2',
    url: 'https://paypal.com',
    startDate: '2023-06-12',
    endDate: 'Present',
    highlights: [
      "Maintained Braintree's internal Ruby on Rails app used to fund over \\$1.5B a day across tens of thousands of merchants",
      'Led development and testing of feature used to withhold \\$2.2M a day from high-risk or delinquent merchants',
      'Automated the migration of \\$57M in held merchant funds into new bank accounts for merchants onboarding to new acquiring banks using a Ruby Rake task scheduled with a Kubernetes CronJob',
      'Increased monthly revenue by \\$670,000 by correcting fee calculations to round to the nearest cent instead of truncating',
      'Prevented the misallocation of \\$6M by identifying and fixing an edge case in the timing of internal routing of funds to different bank accounts',
      // escheatments
      // accounting help
      // sweeps issue from rails 7 upgrade
    ],
  },

  {
    name: 'Poloniex',
    location: 'Remote',
    description: 'Crypto exchange.',
    position: 'Software Engineer Co-op',
    url: 'https://poloniex.com/',
    startDate: '2021-01-06',
    endDate: '2021-08-13',
    highlights: [
      'Eliminated engineering involvement in updating web marketing content by integrating a content management system, fetching content with GraphQL and serving it to the UI from a Node.js backend',
      'Simplified JSON-based rule definitions in a Node.js service used to govern user permissions by adding support for logical negation of rules to its open source rules engine',
      'Integrated internal tools with Zendesk and Jira Service Desk APIs to streamline customer support workflows using PHP, TypeScript, and React',
    ],
  },

  {
    name: 'Sandbox at Northeastern University',
    location: 'Remote',
    description: "Northeastern's premier software consultancy.",
    position: 'Software Developer',
    url: 'https://www.sandboxnu.com/',
    startDate: '2020-09-01',
    endDate: '2021-05-01',
    highlights: [
      'Developed features for a university course planning web application using Ruby on Rails, TypeScript, and React',
      // 'Modeled course schedule data in TypeScript, wrote logic for validating course requirements, and implemented React components to display interactive course plans to users',
      'Wrote a Node.js parser to translate university course documentation PDFs into JSON for internal usage',
      'Provided technical mentoring to teammates on tasks through routine pair-programming',
    ],
  },

  {
    name: 'Teikametrics',
    location: 'Boston, MA',
    description: 'SaaS platform dealing in Amazon ads optimization.',
    position: 'Software Engineer Co-op',
    url: 'https://www.teikametrics.com/',
    startDate: '2020-01-06',
    endDate: '2020-08-21',
    highlights: [
      'Rapidly ramped up onto new technologies, progressing from no Scala, TypeScript, or web development experience to becoming an active contributor to a Scala with Cats backend and a TypeScript, React, and Redux frontend',
      'Spun up APIs in Scala to handle bulk CSV uploads of ad campaign data and expose campaign performance metrics',
      'Implemented interactive React components to guide users through creating ad campaigns and display campaign metrics',
    ],
  },

  {
    name: 'Curriculum Associates',
    location: 'Billerica, MA',
    description: 'Educational software company.',
    position: 'Software Engineer Co-op',
    url: 'https://www.curriculumassociates.com/',
    startDate: '2019-01-14',
    endDate: '2019-06-20',
    highlights: [
      'Updated SQL query templates in Java to retrieve language flags for student assessment data',
      'Drafted and implemented integration tests to ensure correct functionality of query templates',
      'Refactored database schemas with Liquibase and updated existing data for education platform privileges',
    ],
  },

  {
    name: 'Northeastern University College of Computer and Information Science',
    location: 'Boston, MA',
    description: '',
    position: 'Grader for CS2500',
    url: '',
    startDate: '2018-09-01',
    endDate: '2018-12-01',
    highlights: [
      'Reviewed and graded roughly 30 student coding assignments per week written in Racket',
    ],
  },

  {
    name: 'Veoneer',
    location: 'Lowell, MA',
    description: 'Manufacturer of automotive safety systems.',
    position: 'Technical Intern',
    url: 'https://www.veoneer.com',
    startDate: '2018-05-29',
    endDate: '2018-08-10',
    highlights: [
      'Ported over C++ implementation of target track positioner software to a new radar sensor testing suite written in C#',
      'Outlined architecture specifications for new radar units to lay groundwork for future sensor support within testing suite',
    ],
  },
] satisfies ResumeSchema['work'];
