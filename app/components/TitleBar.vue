<script lang="ts" setup>
import type { DropdownMenuItem } from '@nuxt/ui'

const items = ref<DropdownMenuItem[][]>([
  [
    {
      label: 'Account',
      icon: 'i-lucide:user',
      to: 'https://music.youtube.com',
      target: '_blank',
    },
  ],
  [
    {
      label: 'Logout',
      icon: 'i-lucide-log-out',
    },
  ],
])

const { isLogged, userData } = useAuth()
const window = useTauriWindow().getCurrentWindow()

const isMaximized = computedAsync(() => window.isMaximized(), false)
</script>

<template>
  <div data-tauri-drag-region class="w-full flex bg-elevated items-center justify-between" role="button" tabindex="0">
    <UDropdownMenu v-if="isLogged" size="sm" :items>
      <UAvatar icon="lucide:user" :src="userData?.picture" :alt="userData?.username" class="mx-1 bg-default cursor-pointer" />
    </UDropdownMenu>
    <SvgoLogo v-else :filled="true" :font-controlled="false" data-tauri-drag-region class="size-6 mx-2" />
    <UButtonGroup size="lg">
      <UButton icon="lucide:minus" variant="ghost" color="neutral"
               class="md:px-4 rounded-none hover:bg-muted" aria-label="Minimize" @click="window.minimize()" />
      <UButton :icon="isMaximized ? 'lucide:minimize-2' : 'lucide:maximize-2'" variant="ghost" color="neutral"
               class="md:px-4 rounded-none hover:bg-muted" aria-label="Maximize" @click="window.toggleMaximize()" />
      <UButton icon="lucide:x" variant="ghost" color="error"
               class="md:px-4 rounded-none" aria-label="Close" @click="window.close()" />
    </UButtonGroup>
  </div>
</template>
