use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
pub async fn read_text(app: AppHandle) -> Result<String, String> {
    app.clipboard()
        .read_text()
        .map_err(|e| e.to_string())
        .and_then(|opt| opt.ok_or_else(|| "No text in clipboard".to_string()))
}

#[tauri::command]
pub async fn write_text(app: AppHandle, text: String) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn read_image(app: AppHandle) -> Result<Vec<u8>, String> {
    // Note: Tauri clipboard plugin doesn't have direct image support
    // We need to implement this using platform-specific APIs
    // For now, return an error
    Err("Image clipboard not yet implemented".to_string())
}

#[tauri::command]
pub async fn write_image(app: AppHandle, data: Vec<u8>) -> Result<(), String> {
    // Note: Same as read_image
    Err("Image clipboard not yet implemented".to_string())
}
