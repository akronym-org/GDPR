export default defineNuxtConfig({
  extends: '@nuxt-themes/docus',
  telemetry: false,
  ssr: false,
  nitro: {
    preset: 'cloudflare_pages',
  },
})
