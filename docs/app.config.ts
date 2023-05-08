export default defineAppConfig({
  docus: {
    title: 'GDPR',
    description: 'Wrangle Directus permissions with ease',
    url: 'https://akronym.io',
    socials: {
      github: 'akronym-org/gdpr'
    },
    aside: {
      level: 0,
      exclude: []
    },
    header: {
      logo: true,
      showLinkIcon: true
    },
    footer: {
      textLinks: [
        {
          text: 'akronym.io',
          href: 'https://akronym.io'
        }
      ],
    },
    github: {
      edit: true,
      dir: 'docs/content',
      branch: 'main',
      owner: 'akronym-org',
      repo: 'gdpr',
    },
  }
})
