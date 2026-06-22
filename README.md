# NyaHub

> A desktop torrent manager built with Tauri, Preact, and Rust — **educational & demonstrational project only**.

![License](https://img.shields.io/badge/license-MIT-blue)
![Tauri](https://img.shields.io/badge/Tauri-v2-orange)
![Preact](https://img.shields.io/badge/Preact-10-blue)
![Rust](https://img.shields.io/badge/Rust-2021-orange)

---

## ⚠️ Disclaimer

**This project is for educational and demonstrational purposes only.**

- This software is provided "as is" without warranty of any kind
- The author is not responsible for any misuse of this software
- Users are solely responsible for ensuring their use complies with applicable laws and regulations
- This project does not encourage or condone piracy or copyright infringement
- Always respect copyright laws and support content creators

---

## About

NyaHub is a modern desktop application for searching and managing torrents, inspired by [Nyaa.si](https://nyaa.si). Built as a learning project to demonstrate:

- **Tauri v2** desktop app development with Rust backend
- **Preact** with signals for reactive UI
- **TypeScript** for type-safe frontend code
- **CSS custom properties** for theming (dark/light)
- **Responsive design** for desktop and mobile viewports

---

## Features

- 🔍 Search torrents from Nyaa.si
- 📥 Download management with progress tracking
- 🎨 Dark/Light theme toggle
- 📱 Responsive design (desktop + mobile)
- 🌐 Multi-language support (English/Spanish)
- 📋 Markdown rendering for descriptions
- ⚡ Fast search with filters (category, sort, order)

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop Shell | Tauri v2 |
| Frontend | Preact + TypeScript |
| State Management | @preact/signals |
| Build Tool | Vite |
| Backend | Rust (tokio, librqbit) |
| Nyaa API | Custom Rust scraper (nyaapi-rs) |
| Styling | Vanilla CSS with custom properties |

---

## Project Structure

```
nyahub/
├── src/                    # Frontend (Preact + TypeScript)
│   ├── components/         # UI components
│   ├── stores/             # State management (signals)
│   ├── styles/             # CSS modules
│   └── i18n/               # Translations (en/es)
├── src-tauri/              # Tauri v2 backend (Rust)
│   └── src/                # Rust source code
├── nyaa-api/               # Local Rust crate for Nyaa API
│   └── src/                # HTML parser, client, types
└── package.json
```

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri CLI](https://tauri.app/v2/guide/prerequisites/)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/nyahub.git
cd nyahub

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Build

```bash
# Build for production
npm run tauri build
```

---

## Development

```bash
# Start dev server (frontend only)
npm run dev

# Type check
npx tsc --noEmit

# Build frontend
npm run build
```

---

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- [Nyaa.si](https://nyaa.si) — Anime torrent tracker
- [Tauri](https://tauri.app) — Desktop app framework
- [Preact](https://preactjs.com) — Fast React alternative
- [librqbit](https://github.com/ikatson/rqbit) — Rust BitTorrent library

---

**Remember: This is an educational project. Use responsibly and respect copyright laws.**
