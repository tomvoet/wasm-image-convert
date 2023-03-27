<script lang="ts" setup>
const props = defineProps<{
    modelValue: string
    name: string
    label: string
    placeholder: string
}>()

const data = useVModel(props, 'modelValue')
</script>

<template>
    <label :for="name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{{ label }}</label>
    <select :name="name" v-model="data"
        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
        <option selected hidden>{{ placeholder }}</option>
        <slot />
    </select>
</template>