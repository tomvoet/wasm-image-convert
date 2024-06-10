<script setup lang="ts">
const props = defineProps<{
  aspectRatio: number
  sourceDimensions: [number, number]
  size: [number, number]
}>()

const sizeModel = useVModel(props, 'size')

const width = ref(props.size[0])
const height = ref(props.size[1])

let updating = false

watchEffect(() => {
  if (!updating) {
    updating = true
    height.value = Math.round(width.value / props.aspectRatio)
    updating = false
  }
})

watchEffect(() => {
  if (!updating) {
    updating = true
    width.value = Math.round(height.value * props.aspectRatio)
    updating = false
  }
})

watchEffect(() => {
  sizeModel.value = [width.value, height.value]
})
</script>

<template>
  <div class="flex flex-row items-center space-x-2">
    <div class="flex flex-col">
      <label for="width-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Width:</label>
      <input
        id="width-input" v-model="width"
        type="number" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Width"
      >
    </div>
    <UIcon name="i-heroicons-x-mark-20-solid" class="text-gray-400 dark:text-gray-500 mt-7" />
    <div class="flex flex-col">
      <label for="number-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Height:</label>
      <input
        id="number-input" v-model="height"
        type="number" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Height"
      >
    </div>
  </div>
</template>
