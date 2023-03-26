<script setup lang="ts">
const file = ref<File | null>(null)

watchEffect(() => {
    console.log(file.value)
})
</script>

<template>
    <InputsFile v-model="file">Choose File</InputsFile>
</template>