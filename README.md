# NuxtBeats

## Setup

Make sure to install the dependencies:

```bash
bun install
```

## Development Server

Start the development server on `http://localhost:3000`:

```bash
bun run dev:tauri
```

If using wayland and a nvidia gpu use this instead:

```bash
WAYLAND_DEBUG=1 \ # Logs all gtk and wayland interactions
__NV_DISABLE_EXPLICIT_SYNC=1 \
bun run dev:tauri
```

## Production

Build the application for production:

```bash
bun run build
```

Locally preview production build:

```bash
bun run preview
```
