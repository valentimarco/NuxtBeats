<script setup lang="ts">
const appConfigDir = await useTauriPath().appConfigDir()
const { LazyStore } = useTauriStore()
const store = useState<InstanceType<typeof LazyStore>>('store', () => new LazyStore(`${appConfigDir}/store.json`))

await store.value.init()

await callOnce(async () => {
  const cookies = await store.value.get<string>('cookies')
  if (cookies) await commands.instanceYtmusicApi(cookies)
})
</script>

<template>
  <Html class="overflow-x-hidden">
    <Body class="font-sans antialiased">
      <UApp>
        <NuxtLoadingIndicator />
        <UMain>
          <NuxtLayout>
            <NuxtPage />
          </NuxtLayout>
        </UMain>
      </UApp>
    </Body>
  </Html>
</template>
