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

const playlist = computed(() => playlists.value.find(i => i.id === (route.params as { uuid: string }).uuid))
</script>

<template>
  <div class="size-full rounded-none flex flex-col overflow-auto p-2 md:p-4">
    {{ playlist }}
  </div>
</template>
