<script setup lang="ts">
definePageMeta({
  middleware: [async (to) => {
    const { playlists } = await usePlaylists()
    const id = (to.params as { id: string }).id
    if (!playlists.value.some(i => i.id === id))
      throw createError({ statusCode: 404, statusMessage: 'Playlist not found', fatal: true })
  }],
})

const { playlists } = await usePlaylists()
const route = useRoute()

const playlist = computed(() => playlists.value.find(i => i.id === (route.params as { id: string }).id)!)
</script>

<template>
  <div class="grow">
    <div class="flex gap-2 md:gap-4">
      <NuxtImg height="192" width="192" class="size-48 rounded-md" :alt="playlist.name" quality="100"
               :src="playlist.cover[0]" />
      <div>
        <h2 class="font-bold text-lg text-center">
          {{ playlist.name }}
        </h2>
        <h3>{{ playlist.tracks }}</h3>
      </div>
    </div>
  </div>
</template>
