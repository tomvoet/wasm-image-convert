<script setup lang="ts">
import { file } from '@babel/types';

const props = defineProps<{
    modelValue: File | null
}>()

const data = useVModel(props, 'modelValue')

const { open, files } = useFileDialog({
    multiple: false
})

watchEffect(() => {
    if (files.value) {
        data.value = files.value[0]
    }
})


</script>

<template>
    <InputsButton @click="open">Upload File</InputsButton>
</template>

<style scoped></style>