<h1 align="center">NuxtBeats</h1>
<p align="center">
Desktop music player, made with <a href="https://nuxt.com">Nuxt 4</a> and <a href="https://v2.tauri.app">Tauri 2</a>!
</p>

<p float="left">
	<img src="https://img.shields.io/github/package-json/v/valentimarco/NuxtBeats" />
	<img src="https://img.shields.io/github/license/valentimarco/NuxtBeats" />
</p>

## Stack used

- Nuxt v4
- Tauri v2
- NuxtUI v3
- TailwindCSS v4
- TypeScript
- ESLint

## Build

```sh
bun run tauri:build
```

This command will generate the Nuxt static output and bundle the project under `tauri/target`.

## Debug

```sh
bun run tauri:build:debug
```

The same Tauri bundle will generate under `tauri/target`, but with the ability to open the console.

## License

MIT License © 2025-PRESENT [valentimarco](https://github.com/valentimarco)
