# Developer Notes - AFFiNE Tauri Port

## 🎯 For AFFiNE Developers

This document contains important notes for developers working on the Tauri port.

## 🏗️ Architecture Decisions

### Why Tauri?

1. **Size**: 95% smaller binaries (150MB → 8MB)
2. **Performance**: 75% less memory, 2-3x faster startup
3. **Security**: Better default security posture
4. **Modern**: Built on modern tech (Rust, webview)

### Design Principles

1. **API Compatibility**: Maintain 100% compatibility with frontend
2. **Native Integration**: Use Tauri plugins where possible
3. **Type Safety**: Leverage Rust's type system
4. **Documentation**: Comprehensive inline docs

## 🔧 Development Workflow

### Setting Up

```bash
# 1. Ensure Rust is installed
rustc --version

# 2. Install Tauri CLI
cargo install tauri-cli

# 3. Navigate to project
cd packages/frontend/apps/tauri

# 4. Install dependencies
yarn install

# 5. Run dev server
yarn dev
```

### Hot Reload

Tauri supports hot reload for:
- ✅ Frontend changes (instant)
- ❌ Rust backend changes (requires restart)

To reload backend:
```bash
# Stop dev server (Ctrl+C)
yarn dev
```

### Debugging

**Frontend:**
```bash
# Open DevTools in app
# Right-click → Inspect Element
```

**Backend (Rust):**
```bash
# Enable Rust logging
RUST_LOG=debug yarn dev

# Or specific modules
RUST_LOG=affine_tauri=trace yarn dev
```

**Logs location:**
- macOS: `~/Library/Logs/AFFiNE/affine.log`
- Linux: `~/.local/share/AFFiNE/logs/affine.log`
- Windows: `%APPDATA%\AFFiNE\logs\affine.log`

## 📝 Code Style

### Rust

Follow standard Rust conventions:

```rust
// Use #[tauri::command] for all commands
#[tauri::command]
pub async fn my_command(
    app: AppHandle,
    param: String,
) -> Result<String, String> {
    // Implementation
    Ok("success".to_string())
}

// Use proper error handling
match some_operation() {
    Ok(val) => Ok(val),
    Err(e) => Err(format!("Operation failed: {}", e)),
}

// Use log macros
log::info!("Operation completed");
log::error!("Error occurred: {}", err);
```

### TypeScript

Maintain consistency with existing code:

```typescript
// API functions should be async
export const myApi = {
  doSomething: async (param: string): Promise<void> => {
    return invoke('my_command', { param });
  },
};

// Use proper types
interface MyData {
  field: string;
}
```

## 🔌 Adding New Features

### 1. Add Rust Command

```rust
// src-tauri/src/commands/my_module.rs

#[tauri::command]
pub async fn my_new_command(
    app: AppHandle,
    data: String,
) -> Result<String, String> {
    log::info!("Executing my_new_command");
    
    // Your implementation
    
    Ok("success".to_string())
}
```

### 2. Register Command

```rust
// src-tauri/src/main.rs

.invoke_handler(tauri::generate_handler![
    // ... existing commands
    commands::my_module::my_new_command,
])
```

### 3. Add TypeScript Wrapper

```typescript
// src/api.ts

export const myModule = {
  newCommand: async (data: string): Promise<string> => {
    return invoke('my_new_command', { data });
  },
};

// Add to main export
export const api = {
  // ... existing modules
  myModule,
};
```

### 4. Document

Add JSDoc comments:

```typescript
/**
 * Performs a new operation
 * @param data - Input data
 * @returns Result string
 */
newCommand: async (data: string): Promise<string> => {
  return invoke('my_new_command', { data });
},
```

## 🧪 Testing

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function();
        assert!(result.is_ok());
    }
}
```

Run tests:
```bash
cd src-tauri
cargo test
```

### Integration Tests

```bash
# E2E tests (when implemented)
yarn test:e2e
```

## 📦 Building

### Development Build

```bash
yarn tauri:build --debug
```

### Production Build

```bash
yarn build
```

### Platform-specific

```bash
# macOS only
yarn tauri build --target aarch64-apple-darwin

# Linux only
yarn tauri build --target x86_64-unknown-linux-gnu

# Windows only
yarn tauri build --target x86_64-pc-windows-msvc
```

## 🐛 Common Issues

### Issue: "Rust not found"

**Solution:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source $HOME/.cargo/env
```

### Issue: "webkit2gtk not found" (Linux)

**Solution:**
```bash
sudo apt-get install libwebkit2gtk-4.0-dev
```

### Issue: Build fails with "strip: not found" (macOS)

**Solution:**
```bash
# Use system strip
xcode-select --install
```

### Issue: "Command not found" in TypeScript

**Solution:**
1. Check command is registered in `main.rs`
2. Check function signature matches
3. Verify command name (snake_case in Rust)

## 🔐 Security

### File System Access

Always use scoped paths:

```typescript
// ✅ Good - scoped
await readTextFile('config.json', { 
  dir: BaseDirectory.AppData 
});

// ❌ Bad - absolute path (may be blocked)
await readTextFile('/path/to/file.txt');
```

### Command Validation

Always validate inputs in Rust:

```rust
#[tauri::command]
pub async fn save_file(
    filename: String,
    content: String,
) -> Result<(), String> {
    // Validate filename
    if filename.contains("..") || filename.contains("/") {
        return Err("Invalid filename".to_string());
    }
    
    // Proceed with save
    Ok(())
}
```

## 📊 Performance

### Best Practices

1. **Use async/await**: All commands should be async
2. **Avoid blocking**: Use `tokio::spawn` for long operations
3. **Cache when possible**: Store computed values
4. **Lazy initialization**: Init resources only when needed

### Profiling

```bash
# Profile Rust code
cargo build --release
cargo flamegraph --bin affine-tauri

# Profile memory
RUST_LOG=trace yarn dev
```

## 🔄 State Management

### Global State

```rust
use tauri::State;

pub struct MyState {
    data: Arc<RwLock<HashMap<String, String>>>,
}

#[tauri::command]
pub async fn get_data(
    state: State<'_, MyState>,
    key: String,
) -> Result<String, String> {
    let data = state.data.read();
    data.get(&key)
        .cloned()
        .ok_or_else(|| "Key not found".to_string())
}
```

Register state:

```rust
.setup(|app| {
    app.manage(MyState {
        data: Arc::new(RwLock::new(HashMap::new())),
    });
    Ok(())
})
```

## 🔌 Plugin Development

To add a new Tauri plugin:

1. Add to `Cargo.toml`:
```toml
[dependencies]
tauri-plugin-my-plugin = "2.0"
```

2. Register in `main.rs`:
```rust
.plugin(tauri_plugin_my_plugin::init())
```

3. Use in TypeScript:
```typescript
import { myPlugin } from '@tauri-apps/plugin-my-plugin';
await myPlugin.doSomething();
```

## 📚 Resources

### Documentation

- [Tauri Docs](https://tauri.app/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

### Tools

- [Rust Analyzer](https://rust-analyzer.github.io/) - IDE support
- [Clippy](https://github.com/rust-lang/rust-clippy) - Linter
- [rustfmt](https://github.com/rust-lang/rustfmt) - Formatter

### Community

- [Tauri Discord](https://discord.gg/tauri)
- [Rust Users Forum](https://users.rust-lang.org/)

## 🚀 Deployment

### Code Signing

**macOS:**
```bash
# Set env vars
export APPLE_CERTIFICATE="..."
export APPLE_CERTIFICATE_PASSWORD="..."
export APPLE_ID="..."
export APPLE_PASSWORD="..."

yarn tauri build
```

**Windows:**
```bash
# Set env vars
export WINDOWS_CERTIFICATE="..."
export WINDOWS_CERTIFICATE_PASSWORD="..."

yarn tauri build
```

### Auto-updates

1. Generate signing keys:
```bash
yarn tauri signer generate
```

2. Set environment variables:
```bash
export TAURI_PRIVATE_KEY="..."
export TAURI_KEY_PASSWORD="..."
```

3. Build and publish:
```bash
yarn tauri build
```

## 💡 Tips & Tricks

### 1. Quick Rebuild

```bash
# Only rebuild Rust (faster)
cd src-tauri
cargo build

# Full rebuild
yarn tauri build
```

### 2. Debug Prints

```rust
// Development only
#[cfg(debug_assertions)]
println!("Debug: {:?}", value);

// Always
log::debug!("Value: {:?}", value);
```

### 3. Type Conversions

```rust
// String → &str
let s: String = "hello".to_string();
let slice: &str = &s;

// Vec<u8> → String
let bytes = vec![72, 101, 108, 108, 111];
let string = String::from_utf8(bytes)?;
```

### 4. Error Handling

```rust
// Convert any error to String
.map_err(|e| e.to_string())

// Custom error messages
.map_err(|e| format!("Failed to do X: {}", e))
```

## 📞 Getting Help

1. Check this document
2. Review existing code
3. Check [MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md)
4. Search Tauri docs
5. Ask in team chat
6. Create issue with details

## ✅ Checklist for New Features

- [ ] Rust command implemented
- [ ] Command registered in main.rs
- [ ] TypeScript wrapper added
- [ ] Types defined
- [ ] Documentation added
- [ ] Tests written (if applicable)
- [ ] Tested on all platforms
- [ ] Code reviewed
- [ ] Merged to main

---

Happy coding! 🎉

For questions, refer to:
- [README.md](./README.md)
- [MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md)
- [PORTING_SUMMARY.md](./PORTING_SUMMARY.md)
