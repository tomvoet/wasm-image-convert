// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ['@vueuse/nuxt', '@nuxt/eslint', '@nuxt/ui', '@vite-pwa/nuxt'],

  devtools: { enabled: true },

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

  future: {
    compatibilityVersion: 4,
  },

  icon: {
    clientBundle: {
      icons: ['simple-icons:github', 'heroicons:moon-20-solid', 'heroicons:sun-20-solid', 'heroicons:eye-slash-20-solid', 'heroicons:eye-20-solid', 'heroicons:arrow-path'],
    },
  },

  pwa: {
    base: '/wasm-image-convert/',
    scope: '/wasm-image-convert/',
    devOptions: {
      enabled: true,
      suppressWarnings: true,
      type: 'module',
    },
    client: {
      installPrompt: true,
      periodicSyncForUpdates: 60 * 60,
    },
    manifest: {
      name: 'Wasm Image Convert',
      short_name: 'WIC',
      description: 'Convert images using WebAssembly',
      theme_color: '#db2777',
      lang: 'en',
      icons: [
        {
          src: 'pwa-192x192.png',
          sizes: '192x192',
          type: 'image/png',
        },
        {
          src: 'pwa-512x512.png',
          sizes: '512x512',
          type: 'image/png',
        },
        {
          src: 'pwa-512x512.png',
          sizes: '512x512',
          type: 'image/png',
          purpose: 'any',
        },
        {
          src: 'pwa-512x512.png',
          sizes: '512x512',
          type: 'image/png',
          purpose: 'maskable',
        },
      ],
    },
    registerType: 'autoUpdate',
    workbox: {
      navigateFallback: undefined,
    },
  },
})
