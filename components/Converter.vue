<script setup lang="ts">
import ConversionWorker from '@/workers/convert.ts?worker'
// https://github.com/eliaSchenker/nuxt-webworker/blob/main/plugins/sw.ts
import { type WorkerMessage, WorkerMessageType, type WorkerProgress, type WorkerRequest } from '@/workers/convert.d'

const toast = useToast()

const file = ref<File>()

const progress = ref<WorkerProgress>()

const fileEndings = {
  'image/jpeg': 'jpeg',
  'image/png': 'png',
  'image/webp': 'webp',
  'image/gif': 'gif',
  'image/bmp': 'bmp',
  'image/tiff': 'tiff',
  'image/x-icon': 'ico',
}

const outputType = ref('image/jpeg' as keyof typeof fileEndings)

const warning = ref<{
  title: string
  icon: string
  description: string
} | undefined
>()

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

    reader.onloadend = async (e) => {
      const res = e.target?.result as ArrayBuffer

      const arr = new Uint8Array(res)

      try {
        const startTime = performance.now()

        const result = await convert(arr, file.value?.type as keyof typeof fileEndings || 'image/png', outputType.value)

        if (result && result.length)
          startDownload(result, `converted.${fileEndings[outputType.value]}`)

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

function convert(arr: Uint8Array, inputType: keyof typeof fileEndings, outputType: keyof typeof fileEndings): Promise<Uint8Array> {
  // eslint-disable-next-line no-async-promise-executor
  return new Promise(async (resolve, reject) => {
    const params: WorkerRequest = {
      inputFile: arr,
      inputType,
      outputType,
    }

    const { data, post, terminate } = useWebWorker(new ConversionWorker())

    post(params)

    while (true) {
      try {
        await until(data).changed({ timeout: 1000 })
      }
      catch (e) {
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
    <InputsFile v-model="file">
      Choose File
    </InputsFile>
    <div class="flex flex-row items-end space-x-10 pt-3">
      <div class="grow">
        <InputsSelect
          v-model="outputType" name="outputType" placeholder="Select a File Type"
          label="Output File Type"
        >
          <option
            v-for="(imageType, ending) in fileEndings"
            :key="ending" :value="ending"
          >
            {{ imageType }}
          </option>
        </InputsSelect>
      </div>
      <InputsButton @click="startConversion">
        Start Conversion
      </InputsButton>
    </div>
    <UAlert v-if="warning" variant="soft" color="yellow" :title="warning.title" :icon="warning.icon" :description="warning.description" :close-button="{ icon: 'i-heroicons-x-mark-20-solid', color: 'gray', variant: 'link', padded: false }" class="mt-6" @close="warning = undefined" />

    <aside v-if="progress" class="pt-6 w-full text-center">
      <UProgress :value="progress.progress" />
      {{ progress.message }}
    </aside>
  </div>
</template>
