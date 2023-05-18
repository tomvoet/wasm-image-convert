<script setup lang="ts">
import MyWorker from "@/workers/convert.ts?worker"
//https://github.com/eliaSchenker/nuxt-webworker/blob/main/plugins/sw.ts

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

const startConversion = async () => {
    if (file.value) {
        const reader = new FileReader()

        reader.onloadend = async e => {
            const res = e.target?.result as ArrayBuffer

            const arr = new Uint8Array(res)

            try {
                let result = await convert(arr, file.value?.type as keyof typeof fileEndings || "image/png", outputType.value);

                if (result && result.length) startDownload(result, `converted.${fileEndings[outputType.value]}`)
            } catch (e) {
                alert(e)
            }
        }

        reader.readAsArrayBuffer(file.value)
    }
}

const convert = (arr: Uint8Array, inputType: keyof typeof fileEndings, outputType: keyof typeof fileEndings): Promise<Uint8Array> => {
    return new Promise(async (resolve, reject) => {
        const params: {
            inputFile: Uint8Array;
            inputType: string;
            outputType: string;
        } = {
            inputFile: arr,
            inputType: inputType,
            outputType: outputType,
        }

        const { data, post, terminate } = useWebWorker(new MyWorker)

        post(params);

        await until(data).changed();

        terminate();

        if (data.value) {
            resolve(data.value)
        } else {
            reject("Error converting file")
        }
    })
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
            <InputsButton @click="startConversion">Start Conversion</InputsButton>
        </div>
    </div>
</template>