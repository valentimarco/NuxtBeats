<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'

const { playlists } = await usePlaylists()
const localePath = useLocalePath()
const { app } = useAppConfig()
const route = useRoute()
const open = ref(false)

const links = computed(() => [[{
  label: 'Discover',
  icon: 'i-lucide-compass',
  to: '/discover',
  onSelect: () => {
    open.value = false
  },
},
{
  label: 'Recent',
  icon: 'i-lucide-clock',
  to: '/recent',
},
{
  label: 'Favorites',
  icon: 'i-lucide-heart',
  to: '/favorites',
}, {
  label: 'Playlists',
  icon: 'i-lucide-list-music',
  defaultOpen: route.name.startsWith('playlists'),
  children: playlists.value.map(p => ({
    label: p.name,
    avatar: {
      icon: 'i-lucide-music',
      src: p.cover[0]?.includes('gstatic') ? undefined : p.cover[0],
      size: 'md' as const,
      alt: p.name,
    },
    to: localePath({ name: 'playlists-id', params: { id: p.id } }),
  })),
}], [{
  label: 'Feedback',
  icon: 'i-lucide-message-circle',
  to: `${app.repo}/issues`,
  target: '_blank',
}, {
  label: 'Help & Support',
  icon: 'i-lucide-info',
  to: `${app.repo}/wiki`,
  target: '_blank',
}]] satisfies NavigationMenuItem[][])

const groups = computed(() => [{
  id: 'links',
  label: 'Go to',
  items: links.value.flat(),
}])
</script>

<template>
  <UDashboardGroup unit="rem">
    <UDashboardSidebar id="default" v-model:open="open" collapsible resizable
                       class="bg-elevated/25" :ui="{ footer: 'border-t border-default' }">
      <template #header="{ collapsed }">
        <div data-tauri-drag-region class="flex items-center gap-4 px-2">
          <SvgoLogo :filled="true" :font-controlled="false" class="size-6" />
          <p v-if="!collapsed" class="font-bold text-highlighted">
            {{ app.name }}
          </p>
        </div>
      </template>
      <template #default="{ collapsed }">
        <UDashboardSearchButton :collapsed="collapsed" :kbds="['meta', 'F']" />
        <UNavigationMenu :collapsed="collapsed" arrow :items="links[0]" orientation="vertical" tooltip popover />
        <UNavigationMenu :collapsed="collapsed" arrow :items="links[1]" orientation="vertical" tooltip class="mt-auto" />
      </template>
      <template #footer="{ collapsed }">
        <UserMenu :collapsed="collapsed" />
      </template>
    </UDashboardSidebar>
    <UDashboardSearch :groups :colorMode="false" shortcut="meta_f" />
    <slot />
  </UDashboardGroup>
</template>
