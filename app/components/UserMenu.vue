<script setup lang="ts">
import type { DropdownMenuItem } from '@nuxt/ui'

defineProps<{
  collapsed?: boolean
}>()

const colorMode = useColorMode()
const appConfig = useAppConfig()
const { userData, logout } = useAuth()
const colors = ['red', 'orange', 'amber', 'yellow', 'lime', 'green', 'emerald', 'teal', 'cyan', 'sky', 'blue', 'indigo', 'violet', 'purple', 'fuchsia', 'pink', 'rose']
const neutrals = ['slate', 'gray', 'zinc', 'neutral', 'stone']

const user = computed(() => ({
  name: userData.value?.username || 'User',
  avatar: {
    src: userData.value?.picture,
    alt: userData.value?.username || 'User',
  },
}))

const items = computed<DropdownMenuItem[][]>(() => ([[{
  label: 'Account',
  icon: 'i-lucide:user',
  to: 'https://music.youtube.com/@test',
  target: '_blank',
}, {
  label: 'Settings',
  icon: 'i-lucide-settings',
  to: '/settings',
}], [{
  slot: 'appearance',
  type: 'checkbox',
  label: 'Appearance',
  icon: 'i-lucide-sun-moon',
  checked: colorMode.value === 'dark',
  onUpdateChecked(checked) {
    colorMode.preference = checked ? 'dark' : 'light'
  },
  onSelect(e: Event) {
    e.preventDefault()
  },
  ui: { itemTrailingIcon: 'hidden' },
}], [{
  label: 'Log out',
  icon: 'i-lucide-log-out',
  onSelect: logout,
}]]))
</script>

<template>
  <UDropdownMenu :items="items" :content="{ align: 'start' }"
                 :ui="{ content: collapsed ? 'w-48' : 'w-(--reka-dropdown-menu-trigger-width)' }">
    <UButton v-bind="{
      ...user,
      label: collapsed ? undefined : user?.name,
      trailingIcon: collapsed ? undefined : 'i-lucide-chevrons-up-down',
    }" color="neutral" variant="ghost" block :square="collapsed" class="data-[state=open]:bg-elevated" :ui="{
      trailingIcon: 'text-dimmed',
    }" />
    <template #appearance-trailing>
      <USwitch uncheckedIcon="i-lucide-sun" checkedIcon="i-lucide-moon" :modelValue="colorMode.value === 'dark'" />
    </template>
  </UDropdownMenu>
</template>
