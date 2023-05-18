<script lang="ts" setup>
useServerSeoMeta({
  title: "Wasm File Converter",
  description: "Convert different file formats with this free and easy to use app. The app is also very fast and private due to client-side conversions using WebAssembly (WASM). Try it out today!🚀",
})
</script>

<template>
  <Html lang="en" />
  <Title>
    Wasm File Converter
  </Title>
  <div class="bg-white dark:bg-gray-900 h-screen p-3">
    <HeaderBar />
    <Hero />
    <main class="grid place-items-center">
      <Converter />
    </main>
  </div>
</template>
