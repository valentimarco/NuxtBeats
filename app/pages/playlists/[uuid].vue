<script setup lang="ts">
definePageMeta({
  middleware: [async (to) => {
    const { playlists } = await usePlaylists()
    const uuid = (to.params as { uuid: string }).uuid
    if (!playlists.value.some(i => i.id === uuid)) return abortNavigation({ statusCode: 404, statusMessage: "Playlist not found" })
  }],
})

const { playlists } = await usePlaylists()
const route = useRoute()

const playlist = computed(() => playlists.value.find(i => i.id === (route.params as { uuid: string }).uuid)!)
</script>

<template>
  <section class="size-full rounded-none flex flex-col overflow-auto p-2 md:p-4">
    <div class="flex items-end gap-2">
      <NuxtImg height="192" width="192" class="size-48 rounded-md"
               :alt="playlist.name" quality="100" :src="playlist.cover[0]" />
      <h2 class="font-bold text-lg">
        {{ playlist.name }}
      </h2>
      <h3>{{ playlist.description }}</h3>
    </div>
  </section>
</template>
