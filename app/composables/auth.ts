import type { UnlistenFn } from '@tauri-apps/api/event'

interface UserData {
  username: string
  picture?: string
}

export function useAuth() {
  const isLogged = useState('isLogged', () => false)
  const userData = useState<UserData | undefined>('userData')

  const { listen } = useTauriEvent()

  const listeners: UnlistenFn[] = []

  tryOnMounted(async () => {
    listeners.push(await listen('auth:login', async (e) => {
      userData.value = e.payload as UserData
      isLogged.value = true
      await navigateTo('/discover')
    }))

    listeners.push(await listen('auth:logout', async () => {
      userData.value = undefined
      isLogged.value = false
      await navigateTo('/login')
    }))
  })

  tryOnUnmounted(() => {
    while (listeners.length) {
      const unlisten = listeners.pop()
      unlisten && unlisten()
    }
  })

  async function login() {
    return tryCatchTauri(commands.getYtmusicCookies('youtube-login'))
  }

  async function logout() {
    return tryCatchTauri(commands.logoutYtmusic())
  }

  return {
    isLogged: readonly(isLogged),
    userData: readonly(userData),
    login,
    logout,
  }
}
