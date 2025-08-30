<script lang="ts" setup>
const { app } = useAppConfig()
</script>

<template>
  <DashboardPanel>
    <SvgoLogo :filled="true" :font-controlled="false" class="mx-auto my-auto size-40" />
    <h1 class="mx-auto text-3xl sm:text-4xl text-pretty font-bold font-heading md:mb-5">
      {{ app.name }}
    </h1>
  </DashboardPanel>
</template>
