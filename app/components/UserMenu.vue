<script setup lang="ts">
import type { DropdownMenuItem } from '@nuxt/ui'

defineProps<{
  collapsed?: boolean
}>()

const languagesFlags = {
  en: '🇬🇧',
  es: '🇪🇸',
  fr: '🇫🇷',
  de: '🇩🇪',
  it: '🇮🇹',
} as const

const { t } = useI18n()
const colorMode = useColorMode()
const { userData, logout } = useAuth()
const settings = useSettingsStore()
const { changeLocale } = settings
const { locale, locales } = storeToRefs(settings)

const user = computed(() => ({
  name: userData.value?.username || 'User',
  avatar: {
    src: userData.value?.picture,
    alt: userData.value?.username || 'User',
    size: 'md' as const,
  },
}))

const languageOptions = computed<DropdownMenuItem[]>(() => locales.value
  .map(l => ({
    label: `${languagesFlags[l.code]} ${l.name}`,
    type: 'checkbox',
    onSelect: () => changeLocale(l.code),
    checked: l.code === locale.value,
  })),
)

const items = computed<DropdownMenuItem[][]>(() => ([[{
  label: t('user.account'),
  icon: 'i-lucide:user',
  to: 'https://music.youtube.com/@test',
  target: '_blank',
}, {
  label: t('user.settings'),
  icon: 'i-lucide-settings',
  to: '/settings',
}], [{
  slot: 'appearance',
  type: 'checkbox',
  label: t('user.appearance'),
  icon: 'i-lucide-sun-moon',
  checked: colorMode.value === 'dark',
  onUpdateChecked(checked) {
    colorMode.preference = checked ? 'dark' : 'light'
  },
  onSelect(e: Event) {
    e.preventDefault()
  },
  ui: { itemTrailingIcon: 'hidden' },
}, {
  label: t('language.toggle'),
  icon: 'i-tabler-language',
  children: languageOptions.value,
  content: { sideOffset: 12, align: 'center', side: 'right' },
}], [{
  label: t('user.logout'),
  icon: 'i-lucide-log-out',
  onSelect: logout,
}]]))
</script>

<template>
  <UDropdownMenu :items="items" :content="{ align: 'start' }"
                 :ui="{ content: collapsed ? 'w-48' : 'w-(--reka-dropdown-menu-trigger-width)' }">
    <UButton v-bind="{
      ...user,
      size: 'lg',
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
