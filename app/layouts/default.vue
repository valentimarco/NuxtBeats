<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'

const { t } = useI18n()
const { playlists } = await usePlaylists()
const localePath = useLocalePath()
const { app } = useAppConfig()
const route = useRoute()
const open = ref(false)

const playlistItems = computed(() => playlists.value.map(p => ({
  label: p.name,
  avatar: {
    icon: 'i-lucide-music',
    src: p.cover[0]?.includes('gstatic') ? undefined : p.cover[0],
    size: 'md' as const,
    alt: p.name,
  },
  to: localePath({ name: 'playlists-id', params: { id: p.id } }),
})))

const links = computed(() => [[{
  label: t('navigation.discover'),
  icon: 'i-lucide-compass',
  to: '/discover',
  onSelect: () => {
    open.value = false
  },
},
{
  label: t('navigation.recent'),
  icon: 'i-lucide-clock',
  to: '/recent',
},
{
  label: t('navigation.favorites'),
  icon: 'i-lucide-heart',
  to: '/favorites',
}], [{
  label: t('navigation.playlists'),
  type: 'label',
  icon: 'i-lucide-list-music',
}, ...playlistItems.value], [{
  label: t('navigation.feedback'),
  icon: 'i-lucide-message-circle',
  to: `${app.repo}/issues`,
  target: '_blank',
}, {
  label: t('navigation.help'),
  icon: 'i-lucide-info',
  to: `${app.repo}/wiki`,
  target: '_blank',
}]] satisfies NavigationMenuItem[][])

const groups = computed(() => [{
  id: 'links',
  label: t('navigation.links'),
  items: links.value.filter((_i, index) => index !== 1).flat(),
}, {
  id: 'playlists',
  label: t('navigation.playlists'),
  items: playlistItems.value,
}])
</script>

<template>
  <UDashboardGroup unit="rem">
    <UDashboardSidebar id="default" v-model:open="open" collapsible resizable toggleSide="right" :minSize="12" :defaultSize="16"
                       class="bg-elevated/25" :ui="{ header: 'px-2', body: 'px-2', footer: 'px-2 border-t border-default' }">
      <template #header="{ collapsed }">
        <div data-tauri-drag-region class="flex items-center w-full gap-4" :class="{ 'px-2': !collapsed, 'justify-center': collapsed }">
          <SvgoLogo :filled="true" :font-controlled="false" class="size-8 shrink-0" />
          <p v-if="!collapsed" class="font-bold text-lg text-highlighted">
            {{ app.name }}
          </p>
        </div>
      </template>
      <template #default="{ collapsed }">
        <UDashboardSearchButton :collapsed="collapsed" variant="subtle" :label="t('navigation.search')"
                                :kbds="['meta', 'F']" size="lg" :ui="{ base: collapsed ? 'mx-auto' : '' }" />
        <UNavigationMenu :collapsed="collapsed" arrow :items="[...links[0], ...links[1]]" orientation="vertical" tooltip popover
                         :ui="{ linkLeadingIcon: `${collapsed ? 'mx-auto' : ''} size-6` }" />
        <UNavigationMenu :collapsed="collapsed" arrow :items="links[2]" orientation="vertical" tooltip class="mt-auto"
                         :ui="{ linkLeadingIcon: `${collapsed ? 'mx-auto' : ''} size-6` }" />
      </template>
      <template #footer="{ collapsed }">
        <UserMenu :collapsed="collapsed" />
      </template>
    </UDashboardSidebar>
    <UDashboardSearch :groups :colorMode="false" shortcut="meta_f" :placeholder="$t('navigation.search')" />
    <UDashboardPanel :id="route.name" :ui="{ body: 'p-2 sm:p-4' }">
      <template #header>
        <UDashboardNavbar data-tauri-drag-region :title="$t(`navigation.${route.name.replace('-id', '')}`)"
                          :ui="{ root: 'px-2 sm:px-2 bg-elevated/25', title: 'font-medium text-default' }">
          <template #leading>
            <UDashboardSidebarCollapse />
          </template>
          <template #right>
            <WindowButtons />
          </template>
        </UDashboardNavbar>
      </template>
      <template #body>
        <slot />
      </template>
    </UDashboardPanel>
  </UDashboardGroup>
</template>
