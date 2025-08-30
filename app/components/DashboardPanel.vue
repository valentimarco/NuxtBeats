<script lang="ts" setup>
const route = useRoute()
</script>

<template>
  <div class="w-full">
    <UDashboardPanel :id="route.name">
      <template #header>
        <UDashboardNavbar data-tauri-drag-region :title="$t(`dashboard.${route.name}`)"
                          :ui="{ root: 'px-2 sm:px-2 bg-elevated/25', title: 'font-medium text-default' }">
          <template #leading>
            <UDashboardSidebarCollapse />
          </template>
          <template #right>
            <WindowButtons />
          </template>
        </UDashboardNavbar>
      </template>
      <template #body>
        <slot />
      </template>
    </UDashboardPanel>
  </div>
</template>
