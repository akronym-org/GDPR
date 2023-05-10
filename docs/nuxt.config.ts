export default defineNuxtConfig({
  // modules: ['@nuxtjs/color-mode'],
  extends: '@nuxt-themes/docus',
  nitro: {
    preset: 'cloudflare_pages',
  },
  colorMode: {
    preference: 'green', // default value of $colorMode.preference
    fallback: 'light', // fallback value if not system preference found
    hid: 'nuxt-color-mode-script',
    globalName: '__NUXT_COLOR_MODE__',
    componentName: 'ColorScheme',
    classPrefix: '',
    classSuffix: '-mode',
    storageKey: 'nuxt-color-mode'
  }
})
