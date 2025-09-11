<script lang="ts" setup>
const { app } = useAppConfig()
const { favorites, fetchSongsFromPlaylist } = await usePlaylists()

const songs = await computedAsync(async () => {
  const res = await fetchSongsFromPlaylist("LM", 2, null) ?? []
  return favorites.value.songs = res
}, [])


</script>

<template>
  <div class="grow">
    <SvgoLogo :filled="true" :font-controlled="false" class="mx-auto my-auto size-40" />
    <h1 class="mx-auto text-3xl sm:text-4xl text-pretty font-bold font-heading md:mb-5">
      {{ app.name }}
    </h1>
  </div>
</template>
