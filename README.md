# WoW GM Console

A standalone, cross-platform desktop app for issuing GM commands to an [AzerothCore](https://www.azerothcore.org/) (WotLK 3.3.5a) server over its built-in SOAP interface — no WoW client, Docker shell, or server-side console access required.

Built with [Tauri](https://tauri.app/) (Rust backend, Svelte frontend), so it ships as a small native binary rather than an Electron-style bundle.

## Features

- **Multiple server profiles** — save connections for as many servers as you run, switch between them freely. Passwords are stored in your OS's native keychain (Secret Service/libsecret on Linux, Credential Manager on Windows, Keychain on macOS), never in a plaintext config file.
- **Curated GM actions** — buttons/forms for common tasks: revive, add item, set level, set gold, teleport (by name or raw coordinates), kick, ban account, server info, reload table.
- **Raw command console** — type any GM command directly (with or without the leading `.`), see the server's raw response, with command history (↑/↓).
- Curated actions and the raw console share one execution path and one result/audit log, so there's no behavioral difference between them beyond convenience.

## Prerequisites

### On the server

AzerothCore's SOAP interface is disabled by default. Enable it in `worldserver.conf`:

```
SOAP.Enabled = 1
SOAP.IP = 0.0.0.0   # default is 127.0.0.1, which only accepts local connections
SOAP.Port = 7878    # default; adjust if you're running multiple servers
```

Restart the worldserver after changing this. The account you connect with must be GM level 3 (Administrator) or higher:

```
.account set gmlevel <username> 3 -1
```

> **Security note:** SOAP authenticates over HTTP Basic Auth with no built-in TLS — the account password is sent in plaintext on the wire. This is fine on a trusted home LAN; if you need to reach a server over the open internet, tunnel it (SSH, WireGuard, Tailscale, etc.) rather than exposing the SOAP port directly.

### On the machine running this app

- [Rust](https://rustup.rs/) (stable toolchain)
- [Node.js](https://nodejs.org/) (LTS) + npm
- Tauri's platform dependencies — see the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/) for your OS. On Ubuntu/Debian:
  ```bash
  sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
    libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
  ```

## Development

```bash
npm install
npm run tauri dev
```

## Building

```bash
npm run tauri build
```

Produces a native installer/bundle for whatever OS you build on (Linux `.deb`/`.AppImage`/`.rpm`, Windows `.msi`/`.exe`, macOS `.app`/`.dmg`). Cross-compiling for a different OS than the build machine isn't supported by Tauri — use CI (e.g. a GitHub Actions matrix with `ubuntu-latest`/`windows-latest`/`macos-latest`) to produce all three from one source tree.

## Testing

Unit tests (pure logic, no network — SOAP envelope building/parsing, GM command string builders):

```bash
cd src-tauri
cargo test
```

There's also a live integration test suite that exercises the real SOAP client against an actual running server. It's marked `#[ignore]` so it never runs in normal `cargo test`/CI, and takes its target server's details from environment variables rather than anything committed to the repo:

```bash
WOW_GM_TEST_HOST=127.0.0.1 WOW_GM_TEST_PORT=7878 \
WOW_GM_TEST_USERNAME=admin WOW_GM_TEST_PASSWORD=your-password \
cargo test --test live_soap_test -- --ignored
```

## Architecture

```
src-tauri/src/
├── soap/          # SOAP envelope building/parsing + HTTP client (reqwest + quick-xml)
├── profiles/      # server profile storage: JSON metadata + OS-keychain passwords
├── gm_actions.rs  # pure, unit-tested GM command string builders
├── commands/      # Tauri IPC layer — thin wrappers around the above
└── error.rs       # structured error type shared between backend and frontend

src/lib/
├── tauriApi.ts    # typed wrapper around every Tauri invoke() call
├── components/    # ConnectionManager, ServerDashboard, RawConsole, ConsoleOutput
└── *.svelte.ts    # Svelte 5 runes-based shared state (active profile, log)
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
