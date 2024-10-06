// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ['@vueuse/nuxt', '@nuxt/eslint', '@nuxt/ui'],

  app: {
    baseURL: '/wasm-image-convert/',
  },

  experimental: {
    payloadExtraction: false,
  },

  eslint: {
    config: {
      standalone: false,
    },
  },

  compatibilityDate: '2024-10-05',
})