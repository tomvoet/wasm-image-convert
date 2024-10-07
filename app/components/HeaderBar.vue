<script setup lang="ts">
const hideHeroText = useCookie<boolean>('hideHeroText', {
  default: () => false,
})

const toggleHideHeroText = useToggle(hideHeroText)
</script>

<template>
  <header class="flex flex-row justify-end min-h-8">
    <ClientOnly v-if="true">
      <UTooltip
        text="Toggle hero text" class="mr-3"
      >
        <UButton
          :icon="hideHeroText ? 'heroicons:eye-slash-20-solid' : 'heroicons:eye-20-solid'"
          color="gray"
          variant="solid"
          aria-label="Theme"
          @click="toggleHideHeroText()"
        />
      </UTooltip>
      <UTooltip v-if="$pwa?.offlineReady && $pwa.needRefresh" text="Update application" class="mr-3">
        <UButton
          icon="heroicons:arrow-path"
          color="gray"
          variant="solid"
          aria-label="Theme"
          @click="$pwa.updateServiceWorker"
        />
      </UTooltip>
      <UTooltip v-if="!$pwa?.offlineReady && $pwa?.showInstallPrompt" text="Install application" class="mr-3">
        <UButton
          icon="heroicons:arrow-down-20-solid"
          color="gray"
          variant="solid"
          aria-label="Theme"
          @click="$pwa?.install"
        />
      </UTooltip>
      <UTooltip text="Toggle color scheme">
        <InputsColorScheme />
      </UTooltip>
    </ClientOnly>
  </header>
</template>
