# NyaHub

![License](https://img.shields.io/badge/license-MIT-blue)
![Tauri](https://img.shields.io/badge/Tauri-v2-orange)
![Preact](https://img.shields.io/badge/Preact-10-blue)
![Rust](https://img.shields.io/badge/Rust-2021-orange)

<details>
<summary>Table of Contents</summary>

- [About](#about)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Development](#development)
- [Build](#build)
  - [Desktop (Tauri)](#desktop-tauri)
  - [Android (APK / AAB)](#android-apk--aab)
  - [Automated Releases (CI)](#automated-releases-ci)
- [Environment Variables](#environment-variables)
- [Contributing](#contributing)
- [License](#license)
- [Disclaimer](#disclaimer)

</details>

## About

NyaHub is a modern desktop application for searching and managing torrents, inspired by [Nyaa.si](https://nyaa.si). It is built as a learning project to demonstrate:

- Desktop app development with **Tauri v2** and **Rust**
- **Preact** with signals for reactive UI
- **TypeScript** for type-safe frontend code
- **CSS custom properties** for theming (dark/light)
- **Android distribution** via APK and AAB builds
- **CI/CD** with GitHub Actions for multi-platform releases

## Features

- 🔍 Search torrents with filters (category, sort, order)
- 📥 Download management with progress tracking
- 🎨 Dark/Light theme toggle
- 📱 Responsive layout for desktop and mobile
- 🌐 Multi-language support (English / Spanish)
- 📋 Markdown rendering for torrent descriptions
- ⚡ Rust-backed search via `nyaapi-rs`

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop Shell | Tauri v2 |
| Android | Tauri Android + `nyahub` CLI |
| Frontend | Preact + TypeScript |
| State Management | [`@preact/signals`](https://preactjs.com/guide/v10/signals/) |
| Build Tool | Vite |
| Backend | Rust (tokio, [librqbit](https://github.com/ikatson/rqbit)) |
| Nyaa Client | [`nyaapi-rs`](nyaa-api/) |
| Styling | Vanilla CSS with custom properties |

## Project Structure

```
nyahub/
├── src/                    # Frontend (Preact + TypeScript)
│   ├── components/         # Reusable UI components
│   ├── stores/             # State management (signals)
│   ├── styles/             # Global CSS and theme tokens
│   └── i18n/               # Translations (en / es)
├── src-tauri/              # Tauri v2 backend (Rust)
│   └── src/
├── nyaa-api/               # Local Rust crate for Nyaa.si API
│   ├── src/
│   └── Cargo.toml
├── cli/                    # Android build helper CLI (`nyahub`)
│   └── src/main.rs
└── package.json
```

## Prerequisites

- **Node.js** ≥ 18
- **Rust** ≥ 1.70 (stable toolchain)
- **Tauri CLI** ≥ 2 (installed via `npm install -g @tauri-apps/cli` or `npx tauri`)
- **Android SDK** (for Android builds only)

## Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/yourusername/nyahub.git
cd nyahub

# 2. Install frontend dependencies
npm ci

# 3. Run in development mode
npm run tauri dev
```

## Development

```bash
# Frontend only (Vite HMR)
npm run dev

# Type check
npx tsc --noEmit

# Build frontend for production
npm run build
```

## Build

### Desktop (Tauri)

```bash
# Production build (Linux / macOS / Windows)
npx tauri build -- --no-default-features

# Artifacts are output to:
#   src-tauri/target/<target>/release/bundle/
```

### Android (APK / AAB)

NyaHub ships a dedicated CLI helper (`cli/src/main.rs`) built as the `nyahub` binary.

```bash
# Build from the Rust CLI crate
cd cli
cargo run --bin nyahub -- build                  # Release APK (default)
cargo run --bin nyahub -- build --debug          # Debug APK
cargo run --bin nyahub -- build --aab            # AAB for Google Play
cargo run --bin nyahub -- build --apk            # Explicit APK
cargo run --bin nyahub -- build --split-per-abi  # One APK per ABI
cargo run --bin nyahub -- build --clean          # Clean then build
cargo run --bin nyahub -- build --install --run  # Build, install, and launch
```

#### ABI targets

| `--abi` value | Rust target |
|---------------|-------------|
| `arm64` (default family) | `aarch64` |
| `armv7` | `armv7` |
| `i686` | `i686` |
| `x86_64` | `x86_64` |
| `all` | (default — universal) |

The build process:

1. `setup_env` injects `JAVA_HOME`, `ANDROID_HOME`, and `ANDROID_NDK_HOME`.
2. `npx tauri android build --apk` (or `--aab`) compiles the app.
3. The unsigned artifact is zipaligned and signed (`sign_apk`), producing `*-signed.apk`.
4. Output paths are resolved under `src-tauri/gen/android/app/build/outputs/apk/…` or `…/bundle/…`.

#### Installation commands

```bash
# Install a specific APK
cargo run --bin nyahub -- install --apk path/to/app.apk

# Install release APK and launch
cargo run --bin nyahub -- run
```

#### Clean

```bash
cargo run --bin nyahub -- clean       # Tauri Android clean
cargo run --bin nyahub -- clean --deep  # Also drop Gradle cache and build dir
```

#### Environment info

```bash
cargo run --bin nyahub -- env    # Print env + connected devices
cargo run --bin nyahub -- devices
```

### Automated Releases (CI)

Push a tag matching `v*` to trigger the release workflow:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow (`.github/workflows/release.yml`) builds:

- **Linux x86_64** — `.deb` bundle
- **Linux aarch64** — `.deb` bundle (via QEMU)
- **macOS aarch64** — `.app` / `.dmg`
- **macOS x86_64** — `.app` / `.dmg`
- **Windows x86_64** — `.msi` / `.exe`

All artifacts are uploaded and published as a **draft** GitHub Release.

## Environment Variables

Used by `cli/src/main.rs` `setup_env`:

| Variable | Default |
|----------|---------|
| `JAVA_HOME` | `/usr/lib/jvm/java-21-openjdk` |
| `ANDROID_HOME` | `/opt/android-sdk` |
| `ANDROID_NDK_HOME` | `/opt/android-sdk/ndk/27.0.12077973` |

Override them before running `nyahub` if your setup differs.

## Contributing

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

PRs are welcome. Please open an issue first to discuss major changes.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Disclaimer

**This project is for educational and demonstrational purposes only.**

- This software is provided "as is" without warranty of any kind.
- The author is not responsible for any misuse of this software.
- Users are solely responsible for ensuring their use complies with applicable laws and regulations.
- This project does not encourage or condone piracy or copyright infringement.
- Always respect copyright laws and support content creators.
