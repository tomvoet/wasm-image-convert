<script lang="ts" setup>
const colorMode = useColorMode()

const isDark = computed({
  get() {
    return colorMode.value === 'dark'
  },
  set() {
    colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
  },
})
</script>

<template>
  <UButton
    :icon="isDark ? 'heroicons:moon-20-solid' : 'heroicons:sun-20-solid'"
    color="gray"
    variant="solid"
    aria-label="Theme"
    @click="isDark = !isDark"
  />
</template>
