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
      "Maintained Braintree's Ruby on Rails application used to disburse \\$1.5B a day across 15,000 active merchants",
      'Led development, coordinated live testing, and served as a subject matter expert for a feature used to withhold \\$1.5M a day from high-risk or delinquent merchants',
      'Automated the migration of \\$57M out of a closing account for merchants onboarding to new banking partners',
      'Enhanced disbursement reconciliation processes by improving data quality and designing queries in Google BigQuery, reducing unreconciled funds by 88\\% and increasing reconciliation accuracy to 99.7\\%',
      'Increased monthly revenue by \\$650,000 by correcting fee calculations to use rounding instead of truncation',
      'Identified and corrected the misallocation of \\$80M caused by a regression from a Ruby on Rails upgrade',
      'Interviewed intern candidates and onboarded engineers with code walkthroughs and pair programming',
      // 'Prevented the misallocation of \\$6M by identifying and fixing an edge case in the timing of internal routing of funds to different bank accounts',
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
      'Developed internal tools for managing users, markets, and customer support tickets using TypeScript and React',
      'Integrated web application with a content management system using GraphQL and Node.js, enabling marketing teams to update web content without engineering support',
      'Contributed support for rule negation to an open source rules engine powering a Node.js permissions service',
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
      // 'Rapidly ramped up onto new technologies, progressing from no Scala, TypeScript, or web development experience to becoming an active contributor to a Scala with Cats backend and a TypeScript, React, and Redux frontend',
      'Wrote backend APIs in Scala and implemented UIs with TypeScript, React, and Redux to support Amazon ad campaign creation and display campaign performance metrics to users',
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
