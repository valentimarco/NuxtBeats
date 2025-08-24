import { addImports, addTemplate, defineNuxtModule } from 'nuxt/kit'

const tauriModules = [
  { prefix: 'App', importPaths: ['@tauri-apps/api/app'] },
  { prefix: 'Core', importPaths: ['@tauri-apps/api/core'] },
  { prefix: 'Path', importPaths: ['@tauri-apps/api/path'] },
  { prefix: 'Event', importPaths: ['@tauri-apps/api/event'] },
  { prefix: 'Window', importPaths: ['@tauri-apps/api/webviewWindow', '@tauri-apps/api/window'] },
  { prefix: 'Webview', importPaths: ['@tauri-apps/api/webview'] },
  { prefix: 'Shell', importPaths: ['@tauri-apps/plugin-shell'] },
  { prefix: 'Notification', importPaths: ['@tauri-apps/plugin-notification'] },
  { prefix: 'Fs', importPaths: ['@tauri-apps/plugin-fs'] },
  { prefix: 'Store', importPaths: ['@tauri-apps/plugin-store'] },
]

export default defineNuxtModule({
  meta: {
    name: 'nuxt-tauri',
    configKey: 'tauri',
  },
  defaults: {
    prefix: 'useTauri',
  },
  setup(options) {
    tauriModules.forEach(({ prefix, importPaths }) => {
      const composableName = `${options.prefix}${prefix}`
      const templateFile = addTemplate({
        filename: `tauri/${composableName}.ts`,
        write: true,
        getContents: () => `
${importPaths.map((path, index) => `import * as ${prefix}_${index} from '${path}'`).join('\n')}

export function ${composableName}() {
  return {
    ${importPaths.map((_path, index) => `...${prefix}_${index}`).join(',\n    ')}
  }
}`,
      })

      addImports({
        name: composableName,
        from: templateFile.dst,
      })
    })
  },
})
