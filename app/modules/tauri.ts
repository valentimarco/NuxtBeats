import * as tauriApp from '@tauri-apps/api/app'
import * as tauriCore from '@tauri-apps/api/core'
import * as tauriEvent from '@tauri-apps/api/event'
import * as tauriWebviewWindow from '@tauri-apps/api/webviewWindow'
import * as tauriWindow from '@tauri-apps/api/window'
import * as tauriFs from '@tauri-apps/plugin-fs'
import * as tauriNotification from '@tauri-apps/plugin-notification'
import * as tauriShell from '@tauri-apps/plugin-shell'
import * as tauriStore from '@tauri-apps/plugin-store'
import { addImports, addTemplate, defineNuxtModule } from 'nuxt/kit'

const tauriModules = [
  { module: tauriApp, prefix: 'App', importPath: '@tauri-apps/api/app' },
  { module: tauriCore, prefix: 'Core', importPath: '@tauri-apps/api/core' },
  { module: tauriEvent, prefix: 'Event', importPath: '@tauri-apps/api/event' },
  { module: tauriWebviewWindow, prefix: 'WebviewWindow', importPath: '@tauri-apps/api/webviewWindow' },
  { module: tauriWindow, prefix: 'Window', importPath: '@tauri-apps/api/window' },
  { module: tauriShell, prefix: 'Shell', importPath: '@tauri-apps/plugin-shell' },
  { module: tauriNotification, prefix: 'Notification', importPath: '@tauri-apps/plugin-notification' },
  { module: tauriFs, prefix: 'Fs', importPath: '@tauri-apps/plugin-fs' },
  { module: tauriStore, prefix: 'Store', importPath: '@tauri-apps/plugin-store' },
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
    tauriModules.forEach(({ module, prefix, importPath }) => {
      const composableName = `${options.prefix}${prefix}`
      const exports = Object.keys(module).filter(name => name !== 'default')

      const templateFile = addTemplate({
        filename: `tauri/${composableName}.ts`,
        write: true,
        getContents: () => `
import { ${exports.join(', ')} } from '${importPath}'

export function ${composableName}() {
  return {
    ${exports.join(',\n    ')}
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
