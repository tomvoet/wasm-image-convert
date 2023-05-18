// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ["@vueuse/nuxt", "@nuxtjs/tailwindcss"],
  app: {
    baseURL: "/wasm-convert/",
  },
  experimental: {
    payloadExtraction: false,
  },
});
