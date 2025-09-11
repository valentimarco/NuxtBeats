<script setup lang="ts">
const { app } = useAppConfig()
</script>

<template>
  <div class="flex flex-col h-screen">
    <div data-tauri-drag-region class="w-full flex bg-elevated/25 border-b border-default p-2 items-center justify-between">
      <div class="flex items-center gap-4 px-2">
        <SvgoLogo :filled="true" :font-controlled="false" class="size-6" />
        <p class="font-bold text-highlighted">
          {{ app.name }}
        </p>
      </div>
      <WindowButtons />
    </div>
    <UContainer class="relative overflow-hidden grow">
      <slot />
    </UContainer>
  </div>
</template>
