<template>
  <section>
    <div class="py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12">
      <NuxtLink
        to="/"
        class="inline-flex justify-between items-center py-1 px-1 pr-4 mb-7 text-sm text-gray-700 bg-gray-100 rounded-full dark:bg-gray-800 dark:text-white hover:bg-gray-200 dark:hover:bg-gray-700"
        role="alert" aria-label="New feature: Added more file types to image conversion"
      >
        <span class="text-xs bg-primary-600 rounded-full text-white px-4 py-1.5 mr-3">New</span> <span
          class="text-sm font-medium"
        >SVG conversion support added</span>
        <svg class="ml-2 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
          <path
            fill-rule="evenodd"
            d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
            clip-rule="evenodd"
          />
        </svg>
      </NuxtLink>
      <h1
        class="mb-4 text-4xl font-extrabold tracking-tight leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white"
      >
        Wasm Image Converter
      </h1>
      <p class="mb-8 text-lg font-normal text-gray-500 lg:text-xl sm:px-16 xl:px-48 dark:text-gray-400">
        Meet the
        <b class="dark:text-white text-black">WASM Image Converter!</b> Convert your images quickly and securely with our free,
        easy-to-use app. Powered by WebAssembly (WASM), it ensures fast, private client-side conversions.
        <b class="dark:text-white text-black">Give it a try today! 🚀</b>
      </p>
    </div>
  </section>
</template>
