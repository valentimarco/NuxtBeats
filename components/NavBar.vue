<script lang="ts" setup>
const playlists = ref<string[]>([
  "wa",
  "we",
  "we",
  "wo"
])

</script>


<template>
  <nav class="flex flex-col w-64 min-h-screen shadow-lg p-6">
    <div class="flex items-center space-x-2 mb-8">
      <UIcon name="i-lucide-music-2" class="size-5" />
      <h1 class="text-xl font-bold">NuxtBeats</h1>
    </div>
    <UInput icon="i-lucide-search" color="neutral" class="mb-5" variant="subtle" placeholder="Search..." />
    <div class="space-y-6">
      <h3 class="text-xs uppercase text-gray-500 font-bold tracking-wider mb-3">Menu</h3>
      <div class="flex flex-col space-y-2">
        <UButton icon="i-lucide-house" label="Home" :ui="{ label: 'w-full text-center' }" size="xl" />
        <UButton icon="i-lucide-disc-3" label="Discover" :ui="{ label: 'w-full text-center' }" size="xl" />
        <UButton icon="i-lucide-book" label="Library" :ui="{ label: 'w-full text-center' }" size="xl" />
      </div>
      <h3 class="text-xs uppercase text-gray-500 font-bold tracking-wider mb-3">Playlists</h3>
    </div>
    <!-- Generate recent playlists -->
    

  </nav>
</template>
