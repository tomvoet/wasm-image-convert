// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ["@vueuse/nuxt"],
  target: "static",
  app: {
    baseURL: "/wasm-convert/",
  },
});
