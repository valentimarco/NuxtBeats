<script setup lang="ts">
definePageMeta({
  middleware: [async (to) => {
    const { playlists } = await usePlaylists()
    const id = (to.params as { id: string }).id
    if (!playlists.value.some(i => i.id === id)) return abortNavigation({ statusCode: 404, statusMessage: 'Playlist not found' })
  }],
})

const { playlists } = await usePlaylists()
const route = useRoute()

const playlist = computed(() => playlists.value.find(i => i.id === (route.params as { id: string }).id)!)
</script>

<template>
  <section class="size-full rounded-none flex overflow-auto p-2 md:p-4">
    <div class="flex flex-col items-center gap-2">
      <NuxtImg height="192" width="192" class="size-48 rounded-md" :alt="playlist.name" quality="100"
        :src="playlist.cover[0]" />
      <h2 class="font-bold text-lg text-center">
        {{ playlist.name }}
      </h2>
      <h3>{{ playlist.tracks }}</h3>
    </div>
    <div class="flex grow justify-end gap-2 p-2 md:p-4">
      <p>PLACEHOLDER</p>
    </div>
  </section>
</template>
