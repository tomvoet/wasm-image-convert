<script setup lang="ts">
import { Dimensions, type SVGData } from '~/utils/dimensions'

const props = defineProps<{
  file: File | undefined
  svgData: SVGData | undefined
}>()

const data = useVModel(props, 'file')
const svgDataModel = useVModel(props, 'svgData')

function setFile(file: File[] | null | FileList) {
  if (file) {
    data.value = file[0]
  }
}

function onUpdate(e: Event) {
  const target = e.target as HTMLInputElement

  if (target.files) {
    setFile(target.files)
  }
}

const dropZoneRef = ref<HTMLDivElement | null>()

const { isOverDropZone } = useDropZone(dropZoneRef, setFile)

const imgSource = computed(() => {
  if (data.value) {
    return URL.createObjectURL(data.value)
  }
  else {
    return undefined
  }
})

watch(data, () => {
  if (data.value && data.value.type === 'image/svg+xml') {
    const reader = new FileReader()

    reader.onloadend = (e) => {
      const res = e.target?.result as string

      const parser = new DOMParser()

      const doc = parser.parseFromString(res, 'image/svg+xml')

      const svg = doc.querySelector('svg')

      if (svg) {
        const viewBox = svg.getAttribute('viewBox')

        if (viewBox) {
          // aspectRatioModel.value = Dimensions.fromSVGViewBox(viewBox)
          const dimensions = Dimensions.fromSVGViewBox(viewBox)
          svgDataModel.value = dimensions.getSVGData()
        }
      }
    }

    reader.readAsText(data.value)
  }
  else {
    svgDataModel.value = undefined
  }
})
</script>

<template>
  <div class="flex items-center justify-center w-full">
    <label
      ref="dropZoneRef" for="file-dropzone"
      class="flex flex-col items-center justify-center w-full h-64 border-2 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:hover:border-gray-500 dark:hover:bg-gray-600"
      :class="{
        'border-primary-500 dark:border-primary-500': isOverDropZone,
        'border-gray-300 dark:border-gray-600': !isOverDropZone,
      }"
    >
      <div class="flex flex-col items-center justify-center pt-5 pb-6">
        <template v-if="!data">
          <svg
            aria-hidden="true" class="w-10 h-10 mb-3 text-gray-400" fill="none" stroke="currentColor"
            viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
          >
            <path
              stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
            />
          </svg>
          <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
            <span class="font-semibold">Click to upload</span>
            or drag and drop
          </p>
          <p class="text-xs text-gray-500 dark:text-gray-400">Any image file (i.e. png, jpg, jpeg, gif, webp etc.)</p>
        </template>
        <div v-else class="grid place-items-center">
          <div class="relative group" @click.capture.prevent="data = undefined">
            <div
              class="absolute text-white inset-0 hidden group-hover:grid place-items-center backdrop-brightness-50"
            >
              <span>Remove file</span>
            </div>
            <img :src="imgSource || ''" class=" object-scale-down max-h-56 max-w-xl min-h-24 min-w-24">
          </div>
        </div>
      </div>
      <input id="file-dropzone" type="file" accept="image/*" class="hidden" multiple="false" @change="onUpdate">
    </label>
  </div>
</template>
