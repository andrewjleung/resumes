import { ResumeSchema } from '@kurone-kito/jsonresume-types';

// TODO: touch up projects...
// TODO: make work bullet points more technical, say what i actually did to achieve the impact, what technologies did i leverage

export default {
  $schema:
    'https://raw.githubusercontent.com/jsonresume/resume-schema/v1.0.0/schema.json',
  basics: {
    name: 'Andrew Leung',
    label: 'Software Engineer',
    email: 'andrewleung104@gmail.com',
    phone: '(978) 221-8810',
    url: 'https://andrewjleung.me',
    location: {
      // Used for education city.
      city: 'Boston',
      region: 'MA',
    },
    profiles: [
      {
        network: 'LinkedIn',
        username: 'andrewjleung-',
        url: 'https://linkedin.com/in/andrewjleung-',
      },
      {
        network: 'GitHub',
        username: 'andrewjleung',
        url: 'https://github.com/andrewjleung',
      },
    ],
  },
  education: [
    {
      institution: 'Northeastern University',
      url: 'https://www.northeastern.edu/',
      area: 'Computer Science',
      studyType: 'B.S.',
      startDate: '2017-09-01',
      endDate: '2022-05-01',
      score: '3.9/4.0',
    },
  ],
  skills: [
    {
      name: 'Programming Languages',
      level: 'Intermediate',
      keywords: ['TypeScript', 'JavaScript', 'Python', 'Ruby', 'Java', 'SQL'],
    },
    {
      name: 'Tools & Frameworks',
      level: 'Intermediate',
      keywords: [
        'Git',
        'React',
        'Redux',
        'Next.js',
        'Express.js',
        'Ruby on Rails',
        'HTML/CSS',
        'Tailwind',
        'PostgreSQL',
        'Vim',
      ],
    },
  ],
  work: [
    {
      name: 'PayPal',
      location: 'Remote',
      description: 'Braintree post-processing, treasury funding.',
      position: 'Software Engineer 1',
      url: 'https://paypal.com',
      startDate: '2023-06-12',
      endDate: 'Present',
      highlights: [
        "Maintained Kafka consumers, Rake tasks, and Qless jobs comprising Braintree's internal Ruby on Rails application for assessing fees on acquired transactions, paying out transaction funds to merchants, and generating statements and reports",
        'Led development and orchestrated live testing across engineering, product, and account teams for crucial funds disbursement flows used to pause payouts and hold funds for high-risk merchants resulting in a 100\\% success rate across live testing cases',
        'Increased revenue by \\$160,000 a day by correcting percentage-based fee calculations indiscriminantly truncating the decimal of fractional cent amounts to instead round to the nearest cent',
        'Prevented over \\$150,000 in misallocated funds by identifying and fixing an edge case in the timing of  customer transactions ingested via Kafka, movement of funds between internal bank accounts, and regularly-scheduled payouts to merchants',
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
        'Reduced engineering effort spent updating web marketing material by 100\\% by migrating content to DatoCMS content management system and using GraphQL to retrieve and serve content dynamically from a Node.js backend',
        'Broadened the expressiveness of JSON-based rule definitions used to govern user permissions of actions in internal support tools by contributing support for logical negation to open source rules engine used by a Node.js permission verification service',
        // 'maintenance mode toggles? don't really remember...',
        'Constructed Zendesk and Jira Service Desk API integrations and internal web tools in PHP, TypeScript, and React to streamline customer support engineer workflows',
        // 'Improved developer experience for a Node.js permissions service by creating abstractions for rule composition, simplifying testing patterns in Jest, and contributing to underlying open source rules engine library',
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
        'Modeled course schedule data in TypeScript, wrote logic for validating course requirements, and implemented React components to display interactive course plans to users',
        'Wrote a Node.js script to automatically parse university course documentation into JSON for internal usage',
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
        'Demonstrated technical growth starting with no Scala, TypeScript, or web development experience to becoming an active contributor to a Scala with Cats backend and a TypeScript with React/Redux frontend',
        'Built front-end user flows and interactions according to designer specifications with TypeScript, React, HTML, and CSS',
        // 'Surfaced API endpoints in Scala to persist millions of rows of user data in a PostgreSQL database',
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
  ],
  projects: [
    {
      name: 'Bookmark Manager',
      highlights: [
        'Created a self-hostable bookmark URL manager using TypeScript, Next.js, and shadcn/ui components',
        'Built an API for migrating bookmarks from web browsers or other bookmark managers',
        'Persisted data in a PostgreSQL database hosted by Supabase and queried via Drizzle ORM',
      ],
      keywords: [
        'TypeScript',
        'Node.js',
        'React',
        'Next.js',
        'Tailwind',
        'shadcn/ui',
        'Supabase',
        'Drizzle ORM',
        'Jotai',
      ],
      url: 'https://github.com/andrewjleung/linkr',
      startDate: '2023',
    },
    {
      name: 'Random Audio Player',
      highlights: [
        'Created a full-stack web application for demoing and downloading random sounds fetched from the Freesound API',
        'Deployed application frontend, backend, and NGINX reverse proxy to Railway using Docker',
      ],
      keywords: [
        'TypeScript',
        'Node.js',
        'Fastify',
        'React',
        'TailwindCSS',
        'NGINX',
        'Docker',
      ],
      url: 'https://github.com/andrewjleung/raudi',
      startDate: '2022',
    },
    {
      name: 'TheNeedleDrop Review Dataset',
      highlights: [
        "Implemented a Node.js script to fetch and parse metadata from the YouTube Data API in order to generate a CSV dataset of a popular music critic's video music reviews for use in data analysis",
        'Automated real-time updates to a hosted dataset using a PubSubHubBub server deployed to DigitalOcean via GitHub',
      ],
      keywords: [
        'TypeScript',
        'Node.js',
        'Fastify',
        'YouTube Data API',
        'PubSubHubBub',
      ],
      url: 'https://github.com/andrewjleung/fantano-reviews',
      startDate: '2022',
      endDate: '2022',
    },
    {
      name: 'Compiler for Python-like Language',
      highlights: [
        'Wrote a compiler backend in OCaml and a C runtime for a Python-like expression-oriented language',
        'Supported dynamically-typed values including numbers, Booleans, tuples, strings, and first-class functions',
        'Managed program memory with mark-and-sweep garbage collection and register allocation',
        'Wrote over 600 unit tests and end-to-end tests to ensure correctness of compilation using OUnit',
      ],
      keywords: ['OCaml', 'C', 'x86 Assembly'],
      startDate: '2022',
      endDate: '2022',
    },
  ],
  meta: {
    canonical:
      'https://raw.githubusercontent.com/jsonresume/resume-schema/master/resume.json',
    version: 'v1.0.0',
    lastModified: '2017-12-24T15:53:00',
  },
} satisfies ResumeSchema;
