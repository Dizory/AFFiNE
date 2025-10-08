use tauri::AppHandle;

#[tauri::command]
pub async fn change_language(app: AppHandle, language: String) -> Result<(), String> {
    // Store language preference in config
    super::storage::set_config(
        app,
        "language".to_string(),
        serde_json::Value::String(language)
    ).await
}
