<script lang="ts" setup>
import type { UnlistenFn } from '@tauri-apps/api/event'

const { listen } = useTauriEvent()
const window = useTauriWindow().getCurrentWebviewWindow()
const isMaximized = ref(false)
let unlisten: UnlistenFn | undefined

onMounted(async () => {
  isMaximized.value = await window.isMaximized()

  unlisten = await listen('tauri://resize', async () => {
    isMaximized.value = await window.isMaximized()
  })
})

onUnmounted(() => {
  unlisten?.()
})
</script>

<template>
  <UFieldGroup>
    <UButton icon="lucide:minus" variant="soft" color="neutral" class="md:px-4"
             :aria-label="$t('aria.window.minimize')" @click="window.minimize()" />
    <UButton :icon="isMaximized ? 'lucide:minimize-2' : 'lucide:maximize-2'" variant="soft" color="neutral" class="md:px-4"
             :aria-label="$t('aria.window.maximize')" @click="window.toggleMaximize()" />
    <UButton icon="lucide:x" variant="soft" color="error" class="md:px-4"
             :aria-label="$t('aria.window.close')" @click="window.close()" />
  </UFieldGroup>
</template>
