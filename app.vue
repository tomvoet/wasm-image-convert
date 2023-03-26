<script lang="ts" setup>
import init, { greet } from "./wasm/pkg/wasm"

const test = async () => {

  await init()

  greet("Tom")
}
</script>

<template>
  <div>
    <button @click="test">test</button>
  </div>
</template>
