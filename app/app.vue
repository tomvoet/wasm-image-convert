<script lang="ts" setup>
useServerSeoMeta({
  title: 'Wasm Image Converter',
  description: 'Convert different image formats with this free and easy to use app. The app is also very fast and private due to client-side conversions using WebAssembly (WASM). Try it out today!🚀',
  author: 'Tom Voet',
  ogTitle: 'Wasm Image Converter',
  ogDescription: 'Convert different image formats with this free and easy to use app. The app is also very fast and private due to client-side conversions using WebAssembly (WASM). Try it out today!🚀',
  ogUrl: 'https://tomvoet.github.io/wasm-image-convert/',
})

const config = useRuntimeConfig()

const baseUrlWithoutTrailingSlash = config.app.baseURL.replace(/\/$/, '')
</script>

<template>
  <Html lang="en" />
  <Title>
    Wasm File Converter
  </Title>
  <Head>
    <Link rel="icon" :href="`${baseUrlWithoutTrailingSlash}/favicon.ico`" />
    <Link rel="apple-touch-icon" :href="`${baseUrlWithoutTrailingSlash}/apple-touch-icon.png`" />
    <Link rel="mask-icon" :href="`${baseUrlWithoutTrailingSlash}/pwa-maskable-192x192.png`" color="#000000" />
    <Meta name="theme-color" content="#db2777" />
  </Head>
  <NuxtPwaManifest />
  <div class="bg-white dark:bg-gray-900 min-h-screen p-3 overflow-x-hidden max-w-[100vw] flex flex-col">
    <HeaderBar />
    <Hero />
    <main class="grid place-items-center">
      <Converter />
    </main>
    <UNotifications />
    <FooterBar />
  </div>
</template>
