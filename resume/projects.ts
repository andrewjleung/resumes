import { ResumeSchema } from '@kurone-kito/jsonresume-types';

export default [
  {
    name: 'Bookmark Manager',
    highlights: [
      'Created a self-hostable bookmark URL manager using TypeScript, React, Next.js, and shadcn/ui components',
      'Built a tool for importing and re-categorizing bookmarks from Raindrop.io',
      'Persisted data in a PostgreSQL database hosted by Supabase and queried via Drizzle ORM',
      'Implemented exclusive GitHub OAuth login for a predefined user using Supabase Auth',
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
] satisfies ResumeSchema['projects'];
