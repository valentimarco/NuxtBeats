<script setup lang="ts">
import * as locales from '@nuxt/ui/locale'

const { locale, t } = useI18n()
const localePath = useLocalePath()
const { playlists } = await usePlaylists()
console.dir(playlists.value)

const navItems = computed(() => [
  [
    {
      label: 'Home',
      icon: 'i-lucide-home',
      to: localePath('index'),
    },
    {
      label: 'Discover',
      icon: 'i-lucide-compass',
      to: localePath('discover'),
    },
  ],
  [
    {
      label: 'Library',
      icon: 'i-lucide-library',
      type: 'label',
    },
    {
      label: 'Recent',
      icon: 'i-lucide-clock',
    },
    {
      label: 'Favorites',
      icon: 'i-lucide-heart',
      to: localePath('favorites'),
    },
    {
      label: 'Playlists',
      icon: 'i-lucide-list-music',
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
    },
  ],
] satisfies NavigationMenuItem[][])

</script>

<template>
  <div class="flex flex-col h-screen">
    <TitleBar />
    <main class="flex h-full grow">
      <UNavigationMenu orientation="vertical" highlight highlightColor="primary" :items="navItems"
        class="data-[orientation=vertical]:w-full data-[orientation=vertical]:max-w-48 overflow-y-auto select-none" :ui="{
          list: 'space-y-1',
          link: `data-active:bg-(--ui-bg-elevated) rounded-lg`,
          childList: `space-y-1 mt-1`,
        }" />
      <slot />
    </main>
  </div>
</template>
