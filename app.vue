<script lang="ts" setup>
useServerSeoMeta({
  title: 'Wasm Image Converter',
  description: 'Convert different image formats with this free and easy to use app. The app is also very fast and private due to client-side conversions using WebAssembly (WASM). Try it out today!🚀',
  author: 'Tom Voet',
})
</script>

<template>
  <Html lang="en" />
  <Title>
    Wasm File Converter
  </Title>
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
