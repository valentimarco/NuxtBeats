<script setup lang="ts">
import type { CommandPaletteGroup, CommandPaletteItem, NavigationMenuItem } from "@nuxt/ui"

const { t } = useI18n()
const localePath = useLocalePath()
const { version } = useRuntimeConfig().public
const searchOpen = ref(false)
const { playlists } = storeToRefs(useDataStore())

defineShortcuts({
  ctrl_f: () => {
    searchOpen.value = !searchOpen.value
  },
})

const navItems = computed(() => [
  [
    {
      label: "Home",
      icon: "i-lucide-home",
      to: localePath("index"),
    },
    {
      label: "Discover",
      icon: "i-lucide-compass",
      to: localePath("discover"),
    },
  ],
  [
    {
      label: "Library",
      icon: "i-lucide-library",
      type: "label",
    },
    {
      label: "Recent",
      icon: "i-lucide-clock",
      to: localePath("recent"),
    },
    {
      label: "Favorites",
      icon: "i-lucide-heart",
      to: localePath("favorites"),
    },
    {
      label: "Playlists",
      icon: "i-lucide-list-music",
      children: playlists.value.map(p => ({
        label: p.name,
        icon: "i-lucide-music",
        to: localePath({ name: "playlists-uuid", params: { uuid: p.id } }),
      })),
    },
  ],
] satisfies NavigationMenuItem[][])

const searchGroups = computedAsync(async () => [
  {
    id: "sections",
    label: t("sections.title"),
    items: navItems.value.flat().reduce<CommandPaletteItem[]>((acc, item) => {
      if ("children" in item && item.children) {
        acc.push(...item.children.map(child => ({
          ...child,
          onSelect: async () => {
            await navigateTo(child.to)
            searchOpen.value = false
          },
        })))
      }
      else {
        acc.push({
          ...item,
          onSelect: async () => {
            await navigateTo(item.to)
            searchOpen.value = false
          },
        })
      }
      return acc
    }, []),
  },
  {
    id: "actions",
    label: t("actions.title"),
  },
] satisfies CommandPaletteGroup<CommandPaletteItem>[], [])
</script>

<template>
  <UApp>
    <NuxtLoadingIndicator :duration="3000" :throttle="300"
                          color="repeating-linear-gradient(to right, rgb(var(--color-primary-400)) 0%,rgb(var(--color-primary-900)) 100%)" />
    <div class="fixed inset-0 flex scroll-smooth min-h-dvh antialiased transition-colors">
      <aside
        class="flex-col h-full shrink-0 gap-4 w-full items-start scroll-pr-1 transition-[width] duration-300 p-2 max-w-64 flex ring ring-(--ui-border)">
        <div class="flex gap-2 items-end justify-between px-1 w-full">
          <ClientOnly fallbackTag="div">
            <NuxtImg height="40" class="h-10 w-auto" alt="App Logo" quality="100" format="webp" loading="eager"
                     crossorigin="anonymous" />
          </ClientOnly>
          <span class="text-2xs font-bold text-(--ui-text-highlighted)">v{{ version }}</span>
        </div>
        <UModal v-model:open="searchOpen" :title="$t('modals.search.title')"
                :description="$t('modals.search.description')">
          <UButton variant="subtle" color="neutral" block :label="`${$t('button.search')}...`"
                   leadingIcon="i-tabler-search" class="gap-2" @click="searchOpen = true">
            <template #trailing>
              <div class="ms-auto flex gap-0.5 items-center">
                <UKbd>Ctrl</UKbd>
                <UKbd>F</UKbd>
              </div>
            </template>
          </UButton>
          <template #content>
            <UCommandPalette :groups="searchGroups" close class="h-80" @update:open="searchOpen = $event" />
          </template>
        </UModal>
        <UNavigationMenu orientation="vertical" :items="navItems" highlight highlightColor="primary"
                         class="data-[orientation=vertical]:w-full data-[orientation=vertical]:grow overflow-y-auto select-none" :ui="{
                           list: 'space-y-1',
                           link: `data-active:bg-(--ui-bg-elevated) rounded-lg`,
                           childList: `space-y-1 mt-1`,
                         }" />
      </aside>
      <NuxtPage />
    </div>
  </UApp>
</template>
