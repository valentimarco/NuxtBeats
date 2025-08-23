import type { UnlistenFn } from '@tauri-apps/api/event'

interface UserData {
  username: string
  picture?: string
}

export function useAuth() {
  const isLogged = useState('isLogged', () => false)
  const userData = useState<UserData | undefined>('userData')

  const { invoke } = useTauriCore()
  const { listen } = useTauriEvent()

  const listeners: UnlistenFn[] = []

  tryOnMounted(async () => {
    listeners.push(await listen('auth:login', (e) => {
      userData.value = e.payload as UserData
      isLogged.value = true
    }))

    listeners.push(await listen('auth:logout', () => {
      userData.value = undefined
      isLogged.value = false
    }))
  })

  tryOnUnmounted(() => {
    while (listeners.length) {
      const unlisten = listeners.pop()
      unlisten && unlisten()
    }
  })

  async function login() {
    await invoke('get_ytmusic_cookies', { label: 'youtube-login' })
  }

  return {
    isLogged: readonly(isLogged),
    userData: readonly(userData),
    login,
  }
}
