# WoW GM Console

A standalone, cross-platform desktop app for issuing GM commands to an [AzerothCore](https://www.azerothcore.org/) (WotLK 3.3.5a) server over its built-in SOAP interface — no WoW client, Docker shell, or server-side console access required.

Built with [Tauri](https://tauri.app/) (Rust backend, Svelte frontend), so it ships as a small native binary rather than an Electron-style bundle.

## Features

- **Multiple server profiles** — save connections for as many servers as you run, switch between them freely. Passwords are stored in your OS's native keychain (Secret Service/libsecret on Linux, Credential Manager on Windows, Keychain on macOS), never in a plaintext config file.
- **Curated GM actions** — buttons/forms for common tasks: revive, add item, set level, set gold, teleport (by name or raw coordinates), kick, ban account, server info, reload table.
- **Raw command console** — type any GM command directly (with or without the leading `.`), see the server's raw response, with command history (↑/↓).
- Curated actions and the raw console share one execution path and one result/audit log, so there's no behavioral difference between them beyond convenience.
- **Waypoint & item pickers (optional)** — browse/search real named teleport locations and items straight from the server's own world database, rather than needing to already know the exact location name or item ID. Backed by a second, optional, read-only database connection per profile — see [Enabling the database picker](#enabling-the-database-picker-optional) below.

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

#### Enabling the database picker (optional)

The waypoint and item pickers read directly from the server's `game_tele` and `item_template` tables. This is a completely separate, optional connection from SOAP — a profile works fine without it, just without the "Browse…" buttons on Teleport and Add Item.

Rather than pointing the app at your database's root account, create a dedicated **read-only** MySQL user scoped to just the world database:

```sql
CREATE USER IF NOT EXISTS 'gmconsole_ro'@'%' IDENTIFIED BY 'choose-a-strong-password';
GRANT SELECT ON acore_world.* TO 'gmconsole_ro'@'%';
FLUSH PRIVILEGES;
```

(Adjust `acore_world` if your world database schema is named differently.) This way, even if the app's stored credentials were somehow compromised, the account can only ever read world-content tables — it can't modify anything or touch the auth/characters databases.

Then, in the app, edit a server profile and expand "Database connection (optional)" — same LAN-only, no-TLS trust boundary as SOAP applies here too, and the same host machine's firewall/port considerations (this uses MySQL's port, typically `3306`, separate from the SOAP port).

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

Similarly, a live test suite for the database picker backend (also `#[ignore]`d, also env-var-driven) — requires the read-only MySQL user from the [database picker setup](#enabling-the-database-picker-optional) above:

```bash
WOW_GM_TEST_DB_HOST=127.0.0.1 WOW_GM_TEST_DB_PORT=3306 \
WOW_GM_TEST_DB_DATABASE=acore_world WOW_GM_TEST_DB_USERNAME=gmconsole_ro \
WOW_GM_TEST_DB_PASSWORD=your-password \
cargo test --test live_db_test -- --ignored
```

## Architecture

```
src-tauri/src/
├── soap/          # SOAP envelope building/parsing + HTTP client (reqwest + quick-xml)
├── db/            # optional read-only MySQL client (sqlx) for the waypoint/item pickers
├── profiles/      # server profile storage: JSON metadata + OS-keychain passwords
├── gm_actions.rs  # pure, unit-tested GM command string builders
├── commands/      # Tauri IPC layer — thin wrappers around the above
└── error.rs       # structured error type shared between backend and frontend

src/lib/
├── tauriApi.ts     # typed wrapper around every Tauri invoke() call
├── itemClasses.ts  # static WotLK item class/subclass ID -> name lookup (picker filters/labels)
├── components/     # ConnectionManager, ServerDashboard, RawConsole, ConsoleOutput,
│                   # WaypointPicker, ItemPicker
└── *.svelte.ts     # Svelte 5 runes-based shared state (active profile, log)
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
