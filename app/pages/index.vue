<script lang="ts" setup>
const { app } = useAppConfig()
</script>

<template>
  <div class="relative overflow-hidden grow">
    <div class="grid size-full place-content-center gap-y-8">
      <SvgoLogo :filled="true" :font-controlled="false" class="mx-auto size-40" />
      <h1 class="animate-pulse text-3xl sm:text-4xl text-pretty font-bold font-heading md:mb-5">
        {{ app.name }}
      </h1>
    </div>
  </div>
</template>
