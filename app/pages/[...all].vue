<script lang="ts" setup>
definePageMeta({
  layout: 'blank',
})
</script>

<template>
  <UError redirect="/discover" :error="{
    statusCode: 404,
    statusMessage: 'Page not found!',
    message: 'The page you are looking for does not exist.',
  }" />
</template>
