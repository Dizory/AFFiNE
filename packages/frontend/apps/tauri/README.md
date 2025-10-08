# AFFiNE Desktop (Tauri)

This is the Tauri-based desktop application for AFFiNE, replacing the Electron version.

## Benefits over Electron

- **Smaller binary size**: ~5-10 MB vs 80-150 MB
- **Lower memory usage**: ~50-100 MB vs 200-400 MB
- **Faster startup**: System webview instead of bundled Chromium
- **Better security**: Rust-based backend with restricted permissions
- **Native performance**: Direct access to system APIs

## Prerequisites

- **Node.js** >= 18.16.1
- **Rust** >= 1.70 (stable)
- **System dependencies**:
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `build-essential`, `libwebkit2gtk-4.0-dev`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`
  - **Windows**: Microsoft Visual Studio C++ Build Tools

## Development

```bash
# Install dependencies
yarn install

# Run in development mode
yarn dev

# Build for production
yarn build
```

## Architecture

### Rust Backend (`src-tauri/`)

The Rust backend handles:
- Native system integration
- File system operations
- Auto-updates
- System tray
- Deep linking
- Storage management

### Frontend (`src/`)

The TypeScript frontend provides:
- API wrapper compatible with Electron version
- Seamless integration with `@affine/core`
- Type-safe command invocations

## API Compatibility

The Tauri version provides a compatible API with the Electron version:

```typescript
// Instead of Electron IPC
window.affine.api.debug.revealLogFile()

// Uses Tauri commands
invoke('reveal_log_file')
```

## Commands

All Electron IPC handlers have been ported to Tauri commands:

- **Debug**: `reveal_log_file`, `get_log_file_path`
- **UI**: `show_main_window`, `hide_main_window`, `toggle_window`, `get_window_state`, `set_window_state`, `find_in_page`, `stop_find_in_page`
- **Clipboard**: `read_text`, `write_text`, `read_image`, `write_image`
- **Storage**: `get_config`, `set_config`, `delete_config`, `get_shared_storage`, `set_shared_storage`
- **Updater**: `check_for_updates`, `download_update`, `install_update`, `get_current_version`, `get_updater_config`, `set_updater_config`
- **Recording**: `get_recording_status`, `start_recording`, `stop_recording`, `get_recordings`
- **I18n**: `change_language`
- **Worker**: `spawn_worker`, `terminate_worker`

## Plugins

The following Tauri plugins are used:

- `tauri-plugin-updater` - Auto-updates
- `tauri-plugin-dialog` - Native dialogs
- `tauri-plugin-fs` - File system access
- `tauri-plugin-clipboard-manager` - Clipboard operations
- `tauri-plugin-shell` - Shell commands
- `tauri-plugin-notification` - System notifications
- `tauri-plugin-window-state` - Window state persistence
- `tauri-plugin-deep-link` - Deep linking (affine://)
- `tauri-plugin-single-instance` - Single instance enforcement
- `tauri-plugin-http` - HTTP requests

## Building

### Development Build

```bash
yarn tauri dev
```

### Production Build

```bash
yarn tauri build
```

This will create platform-specific installers in `src-tauri/target/release/bundle/`:

- **macOS**: `.app`, `.dmg`
- **Windows**: `.exe`, `.msi`
- **Linux**: `.deb`, `.AppImage`

## Known Limitations

1. **Screen Recording**: Platform-specific implementation needed (macOS ScreenCaptureKit, Windows Desktop Duplication API)
2. **Image Clipboard**: Not yet implemented (requires platform-specific APIs)
3. **Worker Pool**: Simplified implementation (uses Tokio tasks instead of Node.js child processes)

## Migration Notes

### From Electron to Tauri

1. **IPC**: `ipcRenderer.invoke()` → `invoke()`
2. **Events**: `ipcRenderer.on()` → `listen()`
3. **Window**: Native Electron BrowserWindow API → Tauri WebviewWindow API
4. **Menu**: Electron Menu → Tauri Menu/TrayIcon API
5. **Auto-updater**: `electron-updater` → `tauri-plugin-updater`

### Breaking Changes

- No Node.js in renderer process
- No `require()` - only ES modules
- Limited `fs` access (scoped to app directories)
- Different build process and tooling

## Troubleshooting

### Build Errors

**macOS**: Ensure Xcode Command Line Tools are installed:
```bash
xcode-select --install
```

**Linux**: Install required dependencies:
```bash
sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**Windows**: Install Visual Studio Build Tools with C++ workload

### Runtime Errors

Check logs:
```bash
yarn tauri dev --verbose
```

## Contributing

When adding new features:

1. Add Rust command in `src-tauri/src/commands/`
2. Register command in `src-tauri/src/main.rs`
3. Add TypeScript wrapper in `src/api.ts`
4. Update this README with new commands

## License

MIT
