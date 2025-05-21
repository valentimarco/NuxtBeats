import pkg from "./package.json"

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },

  modules: [
    "@nuxt/ui",
    "@nuxt/eslint",
    "@nuxt/image",
    "@nuxt/scripts",
    "@nuxt/test-utils",
    "nuxt-zod-i18n",
    "@vueuse/nuxt",
    "@vueuse/motion/nuxt",
    "@pinia/nuxt",
    "@nuxtjs/i18n",
    "compodium",
  ],

  css: ["~/assets/css/main.css"],

  ssr: false,

  devServer: { host: process.env.TAURI_DEV_HOST || "localhost" },

  vite: {
    // Better support for Tauri CLI output
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://tauri.app/2/reference/environment-variables/
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true,
    },
  },

  experimental: {
    typedPages: true,
  },

  typescript: {
    tsConfig: {
      compilerOptions: {
        types: ["vitest/globals"],
      },
    },
  },

  app: {
    pageTransition: {
      name: "page",
      mode: "out-in",
    },
    layoutTransition: {
      name: "layout",
      mode: "out-in",
    },
  },

  compodium: {
    includeLibraryCollections: true,
    extras: {
      ui: {
        matchColors: true,
      },
    },
  },

  future: {
    compatibilityVersion: 4,
  },

  compatibilityDate: "2024-11-27",

  runtimeConfig: {
    public: {
      version: pkg.version,
    },
  },

  image: {
    quality: 100,
  },

  icon: {
    serverBundle: "local",
  },

  colorMode: {
    classSuffix: "",
    preference: "system",
    fallback: "light",
  },

  zodI18n: {
    dateFormat: {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
    },
    localeCodesMapping: {
      "en-GB": "en",
      "en-US": "en",
      "it-IT": "it",
      "es-ES": "es",
      "fr-FR": "fr",
      "de-DE": "de",
    },
  },

  i18n: {
    baseUrl: "",
    bundle: {
      optimizeTranslationDirective: false,
    },
    experimental: {
      typedPages: true,
      typedOptionsAndMessages: "default",
      generatedLocaleFilePathFormat: "off",
      alternateLinkCanonicalQueries: true,
    },
    compilation: {
      strictMessage: false,
    },
    locales: [
      {
        code: "en",
        language: "en-GB",
        name: "English",
        file: "en-GB.json",
        isCatchallLocale: true,
      },
      {
        code: "it",
        language: "it-IT",
        name: "Italiano",
        file: "it-IT.json",
      },
      {
        code: "es",
        language: "es-ES",
        name: "Español",
        file: "es-ES.json",
      },
      {
        code: "fr",
        language: "fr-FR",
        name: "Français",
        file: "fr-FR.json",
      },
      {
        code: "de",
        language: "de-DE",
        name: "Deutsch",
        file: "de-DE.json",
      },
    ],
    lazy: true,
    defaultLocale: "en",
    strategy: "no_prefix",
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: "i18n_redirected",
      alwaysRedirect: true,
      redirectOn: "root",
    },
  },

  $development: {
    devtools: {
      enabled: true,
      timeline: {
        enabled: true,
      },
    },
  },

  $test: {
    debug: true,
  },
})
