import { version } from './package.json'

export default defineNuxtConfig({
  modules: [
    'nuxt-electron-next',
    'motion-v/nuxt',
    'nuxt-svgo',
    '@vueuse/nuxt',
    '@pinia/nuxt',
    '@nuxtjs/i18n',
    '@nuxt/ui',
    '@nuxt/image',
    '@nuxt/eslint',
    '@compodium/nuxt',
  ],
  electron: {
    build: [
      {
        // Main-Process entry file of the Electron App.
        entry: 'electron/main.ts',
      },
    ],
  },

  app: {
    head: {
      title: 'NuxtBeats',
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      meta: [
        { name: 'format-detection', content: 'no' },
      ],
    },
    pageTransition: {
      name: 'page',
      mode: 'out-in',
    },
    layoutTransition: {
      name: 'layout',
      mode: 'out-in',
    },
  },

  css: [
    '@/assets/css/main.css',
  ],

  icon: {
    serverBundle: 'local',
    clientBundle: {
      scan: true,
      includeCustomCollections: true,
    },
  },

  svgo: {
    autoImportPath: '@/assets/',
  },

  ssr: false,
  dir: {
    modules: 'app/modules',
  },

  vite: {
    build: {
      rollupOptions: {
        external: ['sharp'],
      },
    },
  },
  router: {
    options: {
      scrollBehaviorType: 'smooth',
    },
  },

  eslint: {
    config: {
      standalone: false,
    },
  },

  i18n: {
    experimental: {
      typedPages: true,
      typedOptionsAndMessages: 'default',
    },
    compilation: {
      strictMessage: false,
    },
    locales: [
      {
        code: 'en',
        language: 'en-GB',
        name: 'English',
        file: 'en-GB.json',
        isCatchallLocale: true,
      },
      {
        code: 'it',
        language: 'it-IT',
        name: 'Italiano',
        file: 'it-IT.json',
      },
      {
        code: 'es',
        language: 'es-ES',
        name: 'Español',
        file: 'es-ES.json',
      },
      {
        code: 'fr',
        language: 'fr-FR',
        name: 'Français',
        file: 'fr-FR.json',
      },
      {
        code: 'de',
        language: 'de-DE',
        name: 'Deutsch',
        file: 'de-DE.json',
      },
    ],
    defaultLocale: 'en',
    strategy: 'no_prefix',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      alwaysRedirect: true,
      redirectOn: 'root',
    },
  },

  colorMode: {
    preference: 'system',
    fallback: 'light',
    disableTransition: false,
  },

  devtools: {
    enabled: true,
  },

  experimental: {
    typedPages: true,
    emitRouteChunkError: 'automatic-immediate',
  },

  compatibilityDate: 'latest',

  runtimeConfig: {
    public: {
      version,
    },
  },
})
