<script lang="ts" setup>
import init, { greet } from "./wasm/pkg/wasm"

const isDark = useState<boolean>("dark")

const test = async () => {

  await init()

  greet("Tom")
}
</script>

<template>
  <div class="bg-white dark:bg-gray-900 h-screen p-3">
    <HeaderBar />
    <Hero />
    <div class="w-96">
      <Converter />
    </div>
  </div>
</template>
