<script setup lang="ts">
</script>

<template>
  <section class="size-full rounded-none flex flex-col overflow-auto p-2 md:p-4">
    <h1 class="font-bold text-2xl text-(--ui-primary)">
      Nuxt UI - Starter
    </h1>

    <div class="flex items-center gap-2">
      <UButton label="Documentation" icon="i-lucide-square-play"
               to="https://ui.nuxt.com/getting-started/installation/nuxt" target="_blank" />

      <UButton label="GitHub" color="neutral" variant="outline" icon="i-simple-icons-github"
               to="https://github.com/nuxt/ui" target="_blank" />
    </div>
  </section>
</template>
