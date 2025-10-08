use tauri::{AppHandle, Manager};
use std::path::PathBuf;

#[tauri::command]
pub async fn reveal_log_file(app: AppHandle) -> Result<(), String> {
    let log_dir = app.path().app_log_dir()
        .map_err(|e| e.to_string())?;
    
    let log_file = log_dir.join("affine.log");
    
    if log_file.exists() {
        #[cfg(target_os = "macos")]
        {
            use tauri_plugin_shell::ShellExt;
            app.shell().open("file://".to_string() + &log_file.display().to_string(), None)
                .map_err(|e| e.to_string())?;
        }
        
        #[cfg(target_os = "windows")]
        {
            use tauri_plugin_shell::ShellExt;
            app.shell().command("explorer")
                .args(["/select,", &log_file.display().to_string()])
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        
        #[cfg(target_os = "linux")]
        {
            use tauri_plugin_shell::ShellExt;
            app.shell().open(log_file.parent().unwrap().display().to_string(), None)
                .map_err(|e| e.to_string())?;
        }
        
        Ok(())
    } else {
        Err("Log file not found".to_string())
    }
}

#[tauri::command]
pub async fn get_log_file_path(app: AppHandle) -> Result<String, String> {
    let log_dir = app.path().app_log_dir()
        .map_err(|e| e.to_string())?;
    
    let log_file = log_dir.join("affine.log");
    Ok(log_file.display().to_string())
}
