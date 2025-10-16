# Nihui Addon Updater

<div align="center">

![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)

A modern, lightweight addon updater for World of Warcraft built with Tauri and Vue.js

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Development](#development) • [Contributing](#contributing)

</div>

---

## 📋 Overview

Nihui Addon Updater is a desktop application that automatically manages and updates your World of Warcraft addons directly from GitHub repositories. It provides a clean, user-friendly interface with automatic version checking and one-click updates.

### Key Features

- 🔍 **Auto-detection** - Automatically finds your WoW installation
- 🔄 **Automatic Updates** - Check for addon updates with one click
- 📦 **GitHub Integration** - Pull addons directly from GitHub repositories
- 🎯 **Flexible Update Modes** - Choose between stable releases or development branches
- 🌍 **Multi-language** - Supports English, French, Spanish, German, Italian, and Portuguese
- 🔔 **System Notifications** - Get notified when updates are available
- ⚡ **Background Updates** - Auto-check for updates every hour
- 🎨 **Modern UI** - Beautiful dark theme with responsive design
- 🚀 **Self-updating** - The app automatically updates itself

## 📸 Screenshots

### Main Interface
Clean, modern interface showing addon status at a glance.

### Settings
Configure WoW path, startup behavior, and language preferences.

### Addon Configuration
Per-addon settings for update mode (release vs branch) and branch selection.

## 🚀 Installation

### Download

Download the latest release from the [Releases page](https://github.com/Nihilop/Nihui_AppUpdater/releases).

### Install

1. Download the `.msi` installer
2. Run the installer
3. Launch Nihui Addon Updater
4. The app will auto-detect your WoW installation (or you can set it manually)

## 📖 Usage

### First Launch

1. **WoW Path Configuration**
   - The app will automatically scan for your WoW installation
   - If not found, manually select your `World of Warcraft` folder
   - The path should contain `_retail_/Interface/AddOns`

2. **Check for Updates**
   - The app will automatically check for updates for all installed addons
   - Updates are shown with an orange badge

3. **Install/Update Addons**
   - Click the "Update" button to install the latest version
   - Click "Install" for addons you don't have yet

### Settings

Access settings via the gear icon:

- **Launch on Startup** - Start with Windows
- **Minimize on Startup** - Start minimized to system tray
- **Language** - Choose your preferred language

### Per-Addon Configuration

Click the "Configure" button on any addon to:

- Switch between **Release mode** (stable versions) and **Branch mode** (development builds)
- Select a specific branch to track
- View addon documentation (README)

## 🎯 Supported Addons

This updater currently manages the following Nihui addons:

- **Nihui_uf** - Unit Frames
- **Nihui_ab** - Action Bars
- **Nihui_iv** - Inventory
- **Nihui_cb** - Cast Bars
- **Nihui_np** - Nameplates
- **WaypointUI** - Waypoint UI

## 🔧 Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/) (latest stable)

### Setup

```bash
# Clone the repository
git clone https://github.com/Nihilop/Nihui_AppUpdater.git
cd Nihui_AppUpdater

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

### Project Structure

```
Nihui_AppUpdater/
├── src/                      # Frontend (Vue.js + TypeScript)
│   ├── components/
│   │   └── blocks/          # UI components
│   ├── locales/             # i18n translations
│   ├── services/            # API services
│   └── types.ts             # TypeScript definitions
├── src-tauri/               # Backend (Rust)
│   ├── src/
│   │   └── main.rs          # Tauri commands & logic
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
└── .github/
    └── workflows/           # CI/CD (GitHub Actions)
```

### Technology Stack

- **Frontend**: Vue 3, TypeScript, Tailwind CSS, Reka UI
- **Backend**: Rust, Tauri 2.0
- **Build**: Vite, pnpm
- **CI/CD**: GitHub Actions

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 Version Management

The app version is centralized in `src-tauri/Cargo.toml`. To release a new version:

1. Update the version in `src-tauri/Cargo.toml`
2. Commit and push your changes
3. Create a git tag: `git tag v1.1.0`
4. Push the tag: `git push origin v1.1.0`
5. GitHub Actions will automatically build and create a release

## 🔐 Security

### Signing Keys

Releases are cryptographically signed for security. The signing key is stored securely in GitHub Secrets and never committed to the repository.

### Update Verification

The auto-updater verifies the signature of updates before installation, ensuring that updates come from a trusted source.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/)
- UI components from [Reka UI](https://www.reka-ui.com/)
- Icons from [Lucide](https://lucide.dev/)

## 📞 Support

If you encounter any issues or have questions:

- Open an [issue](https://github.com/Nihilop/Nihui_AppUpdater/issues)
- Check existing issues for solutions

---

<div align="center">

Made with ❤️ for the WoW Community

</div>
