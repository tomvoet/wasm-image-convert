<script setup lang="ts">
import ConversionWorker from '@/workers/convert.ts?worker'
// https://github.com/eliaSchenker/nuxt-webworker/blob/main/plugins/sw.ts
import { type WorkerMessage, WorkerMessageType, type WorkerProgress, type WorkerRequest } from '@/workers/convert.d'
import type { SVGData } from '~/utils/dimensions'
import type { SvgSettings } from '~/utils/settings'
import { outputFileEndings } from '#imports'

const toast = useToast()

const file = ref<File>()
const svgData = ref<SVGData>()

const size = ref<[number, number]>()

watch(svgData, (newData) => {
  if (newData) {
    size.value = [newData.width, newData.height]
  }
  else {
    size.value = undefined
  }
})

const progress = ref<WorkerProgress>()

const outputType = ref('image/jpeg' as keyof typeof outputFileEndings)

const warning = ref<{
  title: string
  icon: string
  description: string
}>()

watch(outputType, () => {
  if (outputType.value === 'image/x-icon') {
    warning.value = {
      title: 'Warning',
      icon: 'i-heroicons-exclamation-circle',
      description: 'Converting an image to an icon file will resize it to 256x256',
    }
  }
  else {
    warning.value = undefined
  }
})

async function startConversion() {
  if (file.value) {
    const reader = new FileReader()

    console.log(file.value)

    reader.onloadend = async (e) => {
      const res = e.target?.result as ArrayBuffer

      const arr = new Uint8Array(res)

      try {
        const startTime = performance.now()

        if (!file.value) {
          toast.add({
            title: 'Error',
            icon: 'i-heroicons-exclamation-circle',
            description: 'No file selected',
          })

          return
        }

        console.log(getMimeType(file.value))

        const result = await convert(arr, getMimeType(file.value), outputType.value)

        if (result && result.length)
          startDownload(result, `converted.${inputFileEndings[outputType.value]}`)

        const endTime = performance.now()

        toast.add({
          title: 'Success',
          icon: 'i-heroicons-check-circle',
          description: `Conversion completed successfully in ${(endTime - startTime).toFixed(2)}ms`,
        })
      }
      catch (e) {
        let error = (e as any).message || (e as any).toString()

        error = error.replace(/^Error: /, '')

        toast.add({
          title: 'Error',
          icon: 'i-heroicons-exclamation-circle',
          description: error,
          actions: [{
            leadingIcon: 'i-heroicons-arrow-path',
            label: 'Retry',
            click: () => startConversion(),
          }],
        })

        progress.value = undefined
      }
    }

    reader.readAsArrayBuffer(file.value)
  }
}

function convert(arr: Uint8Array, inputType: MimeTypes, outputType: keyof typeof outputFileEndings): Promise<Uint8Array> {
  // eslint-disable-next-line no-async-promise-executor
  return new Promise(async (resolve, reject) => {
    const params: WorkerRequest = {
      inputFile: arr,
      inputType,
      outputType,
    }

    if (svgData.value && size.value) {
      Object.assign(params, {
        settings: {
          type: 'svg',
          width: size.value[0],
          height: size.value[1],
        } as SvgSettings,
      })
    }

    const { data, post, terminate } = useWebWorker(new ConversionWorker())

    post(params)

    while (true) {
      try {
        await until(data).changed({ timeout: 1000 })
      }
      // eslint-disable-next-line unused-imports/no-unused-vars
      catch (_e) {
        reject(new Error('Conversion timed out'))
      }

      const res = data as Ref<WorkerMessage>

      switch (res.value.type) {
        case WorkerMessageType.DONE:
          resolve(res.value.payload.data as Uint8Array)
          break
        case WorkerMessageType.ERROR:
          reject(res.value.payload.error)
          break
        case WorkerMessageType.PROGRESS:
          progress.value = res.value.payload
          break
      }

      if (res.value.type === WorkerMessageType.DONE || res.value.type === WorkerMessageType.ERROR) {
        terminate()
        break
      }
    }
  })
}

watch(file, () => {
  progress.value = undefined
})
</script>

<template>
  <div class="w-3/4 max-w-2xl">
    <InputsFile v-model:file="file" v-model:svg-data="svgData">
      Choose File
    </InputsFile>
    <div class="flex flex-row items-end space-x-10 pt-3">
      <div class="grow">
        <InputsSelect v-model="outputType" name="outputType" placeholder="Select a File Type" label="Output File Type">
          <option v-for="(imageType, ending) in outputFileEndings" :key="ending" :value="ending">
            {{ imageType }}
          </option>
        </InputsSelect>
      </div>
      <InputsButton @click="startConversion">
        Start Conversion
      </InputsButton>
    </div>
    <UAlert
      v-if="warning" variant="soft" color="yellow" :title="warning.title" :icon="warning.icon"
      :description="warning.description"
      :close-button="{ icon: 'i-heroicons-x-mark-20-solid', color: 'gray', variant: 'link', padded: false }"
      class="mt-6" @close="warning = undefined"
    />
    <div
      v-if="svgData !== undefined && size !== undefined"
      class="mt-6 block p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 w-full"
    >
      <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
        SVG Settings
      </h5>
      <InputsSize
        v-model:size="size" :aspect-ratio="svgData ? svgData.aspectRatio : 2"
        :source-dimensions="[svgData ? svgData.width : 0, svgData ? svgData.height : 0]"
      />
    </div>
    <aside v-if="progress" class="pt-6 w-full text-center">
      <UProgress :value="progress.progress" />
      {{ progress.message }}
    </aside>
  </div>
</template>
