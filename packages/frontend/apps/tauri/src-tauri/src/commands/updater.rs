use tauri::{AppHandle, Manager};
use tauri_plugin_updater::UpdaterExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub current_version: String,
    pub date: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdaterConfig {
    pub auto_check: bool,
    pub auto_download: bool,
    pub check_on_startup: bool,
}

#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> Result<Option<UpdateInfo>, String> {
    let updater = app.updater_builder().build().map_err(|e| e.to_string())?;
    
    match updater.check().await {
        Ok(Some(update)) => {
            Ok(Some(UpdateInfo {
                version: update.version.clone(),
                current_version: update.current_version.clone(),
                date: update.date.clone(),
                body: update.body.clone(),
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn download_update(app: AppHandle) -> Result<(), String> {
    let updater = app.updater_builder().build().map_err(|e| e.to_string())?;
    
    if let Some(update) = updater.check().await.map_err(|e| e.to_string())? {
        // Download and install
        let mut downloaded = 0u64;
        
        update.download_and_install(
            |chunk_length, content_length| {
                downloaded += chunk_length;
                log::info!("Downloaded {} of {:?}", downloaded, content_length);
            },
            || {
                log::info!("Download finished");
            },
        ).await.map_err(|e| e.to_string())?;
        
        Ok(())
    } else {
        Err("No updates available".to_string())
    }
}

#[tauri::command]
pub async fn install_update(app: AppHandle) -> Result<(), String> {
    // In Tauri 2, the update is installed automatically after download
    // We just need to restart the app
    app.restart();
}

#[tauri::command]
pub async fn get_current_version(app: AppHandle) -> Result<String, String> {
    Ok(app.package_info().version.to_string())
}

#[tauri::command]
pub async fn get_updater_config(app: AppHandle) -> Result<UpdaterConfig, String> {
    // Read from config storage
    let config = super::storage::get_config(app, Some("updater".to_string())).await?;
    
    if config.is_null() {
        return Ok(UpdaterConfig {
            auto_check: true,
            auto_download: false,
            check_on_startup: true,
        });
    }
    
    serde_json::from_value(config).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_updater_config(app: AppHandle, config: UpdaterConfig) -> Result<(), String> {
    let value = serde_json::to_value(config).map_err(|e| e.to_string())?;
    super::storage::set_config(app, "updater".to_string(), value).await
}
