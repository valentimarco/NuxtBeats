<script setup lang="ts">
const { playlists } = storeToRefs(useDataStore())
const route = useRoute()

const playlist = computed(() => playlists.value.find(i => i.id === (route.params as Record<string, any>).uuid))
</script>

<template>
  <div class="size-full rounded-none flex flex-col overflow-auto p-2 md:p-4">
    {{ playlist }}
  </div>
</template>
