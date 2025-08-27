<script lang="ts" setup>
definePageMeta({
  layout: 'blank',
})

const { WebviewWindow } = useTauriWindow()
const { login } = useAuth()
const toast = useToast()

async function loginToYoutubeMusic() {
  const webview = new WebviewWindow('youtube-login', {
    title: 'Login to Youtube Music',
    url: 'https://accounts.google.com/ServiceLogin?continue=https%3A%2F%2Fmusic.youtube.com',
    parent: 'main',
    width: 800,
    height: 600,
    minimizable: false,
    maximizable: false,
    closable: true,
  })

  webview.listen('tauri://close-requested', async () => {
    await login()
    await webview.destroy()
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
