<script lang="ts" setup>

</script>

<template>
  <div class="flex min-h-dvh bg-zinc-900 text-white scroll-smooth antialiased transition-colors">

    <NavBar />
    <div class="flex flex-col flex-1 rounded-lg border">
      <slot />
    </div>

    <MusicPlayer />


  </div>

  <!-- <UNotifications /> -->
</template>
