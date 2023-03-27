<script setup lang="ts">
import init, { greet, convert_image } from "@/wasm/pkg/wasm"

const file = ref<File | null>(null)

const fileEndings = {
    "image/jpeg": "jpeg",
    "image/png": "png",
    "image/webp": "webp",
    "image/gif": "gif",
    "image/bmp": "bmp",
    "image/tiff": "tiff",
    "image/x-icon": "ico",
}

const outputType = ref("image/jpeg" as keyof typeof fileEndings)

watchEffect(() => {
    console.log(file.value)
})

const convert = async () => {
    if (file.value) {
        await init()

        const reader = new FileReader()
        reader.onloadend = e => {
            const res = e.target?.result as ArrayBuffer

            const arr = new Uint8Array(res)

            const result = convert_image(arr, file.value?.type || "image/png", outputType.value)

            console.log("ERGEBNIS", result)

            startDownload(result, `converted.${fileEndings[outputType.value]}`)
        }
        reader.readAsArrayBuffer(file.value)
    }
}
</script>

<template>
    <div class="w-3/4 max-w-2xl">
        <InputsFile v-model="file">Choose File</InputsFile>
        <div class="flex flex-row items-end space-x-10 pt-3">
            <div class="grow">
                <InputsSelect name="outputType" placeholder="Select a File Type" label="Output File Type"
                    v-model="outputType">
                    <option v-for="(imageType, ending) in fileEndings" :value="ending">{{ imageType }}</option>
                </InputsSelect>
            </div>
            <InputsButton @click="convert">Start Conversion</InputsButton>
        </div>
    </div>
</template>