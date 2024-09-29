import { ResumeSchema } from '@kurone-kito/jsonresume-types';

export default {
  name: 'Andrew Leung',
  label: 'Software Engineer',
  email: 'andrewleung104@gmail.com',
  phone: '(978) 221-8810',
  url: 'https://andrewjleung.me',
  location: {
    // Using for education city.
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
} satisfies ResumeSchema['basics'];
