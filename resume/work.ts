import { ResumeSchema } from '@kurone-kito/jsonresume-types';

export default [
  {
    name: 'PayPal',
    location: 'Remote',
    description: 'Braintree post-processing, treasury funding.',
    position: 'Software Engineer 1',
    url: 'https://paypal.com',
    startDate: '2023-06-12',
    endDate: 'Present',
    highlights: [
      "Maintained Braintree's internal Ruby on Rails application used to assess fees, generate statements and reports, and disburse over \\$1.5B daily across tens of thousands of merchants",
      'Led development and live testing of a feature used to pause disbursements and hold funds for high-risk or delinquent merchants, achieving a 100\\% success rate during live testing and facilitating the processing of \\$2M in held funds daily',
      'Automated the migration of \\$55M in held merchant funds into new interest-generating bank accounts using a Ruby Rake task scheduled with a Kubernetes CronJob',
      'Increased monthly revenue by \\$450,000 by correcting fee calculations to round to the nearest cent instead of truncating',
      'Prevented the potential misallocation of \\$500,000 by resolving an edge case in the timing of transaction ingestion, transfers between internal bank accounts, and transitioning merchants onto new disbursement flows',
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
      'Eliminated manual engineering effort spent updating web marketing material by migrating content to DatoCMS, fetching it with GraphQL, and serving it from a Node.js backend, enabling fully dynamic content management',
      'Expanded the expressiveness of JSON-based rule definitions in a Node.js permissions service used to govern user permissions across internal applications by adding support for logical negation of rules to its open source rules engine',
      // 'maintenance mode toggles? don't really remember...',
      'Constructed Zendesk and Jira Service Desk API integrations and internal web tools using PHP, TypeScript, and React to streamline customer support engineer workflows',
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
      'Quickly ramped up onto new technologies, progressing from no Scala, TypeScript, or web development experience to becoming an active contributor to a Scala with Cats backend and TypeScript, React, and Redux frontend',
      'Reduced effort for users modifying ad campaign targets by implementing a React UI and Scala API for bulk CSV uploads',
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
