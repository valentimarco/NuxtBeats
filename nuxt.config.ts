import { version } from './package.json'

export default defineNuxtConfig({
  modules: [
    '@vueuse/nuxt',
    '@nuxt/ui',
    'nuxt-svgo',
    '@nuxt/eslint',
  ],

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
    customCollections: [
      {
        prefix: 'local',
        dir: './app/assets/icons',
      },
    ],
  },

  svgo: {
    autoImportPath: '@/assets/',
  },

  ssr: false,

  dir: {
    modules: 'app/modules',
  },

  imports: {
    presets: [
      {
        from: 'zod',
        imports: [
          'z',
          {
            name: 'infer',
            as: 'zInfer',
            type: true,
          },
        ],
      },
    ],
  },

  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
      hmr: {
        protocol: 'ws',
        host: '0.0.0.0',
        port: 3001,
      },
      watch: {
        ignored: ['**/src-tauri/**'],
      },
    },
    build: {
      rollupOptions: {
        external: ['sharp'],
      },
    },
  },

  devServer: {
    host: process.env.TAURI_DEV_HOST || 'localhost',
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

  colorMode: {
    disableTransition: false,
  },

  devtools: {
    enabled: false,
  },

  experimental: {
    typedPages: true,
  },

  compatibilityDate: 'latest',

  runtimeConfig: {
    public: {
      version,
    },
  },
})
