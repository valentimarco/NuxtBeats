<script lang="ts" setup>
const window = useTauriWindow().getCurrentWebviewWindow()

const isMaximized = ref(false)

onMounted(async () => {
  isMaximized.value = await window.isMaximized()
})

async function toggleMaximize() {
  await window.toggleMaximize()
  isMaximized.value = await window.isMaximized()
}
</script>

<template>
  <UFieldGroup>
    <UButton icon="lucide:minus" variant="ghost" color="neutral" class="md:px-4"
             :aria-label="$t('aria.window.minimize')" @click="window.minimize()" />
    <UButton :icon="isMaximized ? 'lucide:minimize-2' : 'lucide:maximize-2'" variant="ghost" color="neutral" class="md:px-4"
             :aria-label="$t('aria.window.maximize')" @click="toggleMaximize()" />
    <UButton icon="lucide:x" variant="ghost" color="error" class="md:px-4"
             :aria-label="$t('aria.window.close')" @click="window.close()" />
  </UFieldGroup>
</template>
