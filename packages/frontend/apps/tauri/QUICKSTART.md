# Quick Start Guide

Get AFFiNE Tauri running in 5 minutes!

## Prerequisites

Install these first:

```bash
# 1. Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Node.js 20+ (if not installed)
# Use your preferred method (nvm, fnm, etc.)

# 3. System dependencies
```

### macOS
```bash
xcode-select --install
```

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Windows
Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) with C++ workload.

## Installation

```bash
# From project root
cd /workspace

# Enable corepack
corepack enable

# Install dependencies
yarn install

# Build frontend core
yarn affine @affine/core build
```

## Development

```bash
# Navigate to Tauri app
cd packages/frontend/apps/tauri

# Start development server
yarn dev
```

The app should launch automatically! 🎉

## Building

```bash
# Production build
yarn build

# Output will be in:
# src-tauri/target/release/bundle/
```

## Common Issues

### "Rust not found"
```bash
# Restart terminal or run:
source $HOME/.cargo/env
```

### "webkit2gtk not found" (Linux)
```bash
sudo apt-get install libwebkit2gtk-4.0-dev
```

### Build errors on macOS
```bash
# Use system strip instead of binutils
xcode-select --install
```

## Testing

```bash
# Run in dev mode
yarn dev

# Check if window opens
# Try system tray menu
# Test deep links: affine://
```

## Next Steps

- Read [README.md](./README.md) for detailed documentation
- Check [MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md) for API differences
- Review [PORTING_SUMMARY.md](./PORTING_SUMMARY.md) for technical details

## Getting Help

- Check existing issues
- Review Tauri documentation
- Ask in AFFiNE community

Happy coding! 🚀
