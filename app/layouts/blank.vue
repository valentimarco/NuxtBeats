<template>
  <div class="flex flex-col h-screen">
    <TitleBar />
    <UContainer class="relative overflow-hidden grow">
      <slot />
    </UContainer>
  </div>
</template>
