<script setup lang="ts">
const { favorites } = await usePlaylists()
</script>

<template>
  <section class="size-full rounded-none flex flex-col overflow-auto p-2 md:p-4">
    {{ favorites }}
  </section>
</template>
