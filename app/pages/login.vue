<script lang="ts" setup>
definePageMeta({
  layout: 'blank',
})

const { WebviewWindow } = useTauriWebview()
const toast = useToast()

async function loginToYoutubeMusic() {
  const webview = new WebviewWindow(Date.now().toString(), {
    title: 'Youtube Login',
    url: 'https://music.youtube.com',
    width: 800,
    height: 600,
    visible: true,
    focus: true,
    resizable: false,
  })

  webview.listen('tauri://error', (e) => {
    toast.add({
      title: 'Error',
      description: String(e.payload),
      color: 'error',
    })
  })
}
</script>

<template>
  <div class="grid size-full place-content-center gap-y-8">
    <h1 class="text-3xl sm:text-4xl text-pretty font-bold font-heading">
      Login to Youtube Music
    </h1>
    <UButton variant="subtle" icon="lucide:cookie" class="justify-center" @click="loginToYoutubeMusic">
      Obtain Cookies
    </UButton>
  </div>
</template>
