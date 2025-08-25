import type { Locale } from '#i18n'

export const useSettingsStore = defineStore('Settings', () => {
  const colorMode = useColorMode()

  const isDark = computed({
    get() {
      return colorMode.value === 'dark'
    },
    set(_isDark) {
      colorMode.preference = _isDark ? 'dark' : 'light'
    },
  })
  const toggleDark = useToggle(isDark)

  const { locale, locales, setLocale, setLocaleCookie } = useI18n()
  const switchLocalePath = useSwitchLocalePath()

  const currentLocale = computed(() =>
    locales.value.find(l => l.code === locale.value) ?? locales.value[0]!,
  )

  const changeLocale = (e: Locale) => {
    switchLocalePath(e)
    setLocaleCookie(e)
    setLocale(e)
  }

  return {
    isDark,
    locale,
    locales,
    currentLocale,
    toggleDark,
    changeLocale,
  }
})
