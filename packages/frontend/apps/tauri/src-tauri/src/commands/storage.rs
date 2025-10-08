use tauri::{AppHandle, Manager};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn get_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data = app.path().app_config_dir()
        .map_err(|e| e.to_string())?;
    
    fs::create_dir_all(&app_data).map_err(|e| e.to_string())?;
    Ok(app_data.join("config.json"))
}

fn get_shared_storage_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data = app.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    
    fs::create_dir_all(&app_data).map_err(|e| e.to_string())?;
    Ok(app_data.join("shared-storage.json"))
}

fn read_json_file(path: &PathBuf) -> Result<Value, String> {
    if !path.exists() {
        return Ok(Value::Object(Default::default()));
    }
    
    let content = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;
    
    serde_json::from_str(&content)
        .map_err(|e| e.to_string())
}

fn write_json_file(path: &PathBuf, data: &Value) -> Result<(), String> {
    let content = serde_json::to_string_pretty(data)
        .map_err(|e| e.to_string())?;
    
    fs::write(path, content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_config(app: AppHandle, key: Option<String>) -> Result<Value, String> {
    let config_path = get_config_path(&app)?;
    let config = read_json_file(&config_path)?;
    
    if let Some(k) = key {
        Ok(config.get(&k).cloned().unwrap_or(Value::Null))
    } else {
        Ok(config)
    }
}

#[tauri::command]
pub async fn set_config(app: AppHandle, key: String, value: Value) -> Result<(), String> {
    let config_path = get_config_path(&app)?;
    let mut config = read_json_file(&config_path)?;
    
    if let Value::Object(ref mut map) = config {
        map.insert(key, value);
    }
    
    write_json_file(&config_path, &config)
}

#[tauri::command]
pub async fn delete_config(app: AppHandle, key: String) -> Result<(), String> {
    let config_path = get_config_path(&app)?;
    let mut config = read_json_file(&config_path)?;
    
    if let Value::Object(ref mut map) = config {
        map.remove(&key);
    }
    
    write_json_file(&config_path, &config)
}

#[tauri::command]
pub async fn get_shared_storage(app: AppHandle, key: Option<String>) -> Result<Value, String> {
    let storage_path = get_shared_storage_path(&app)?;
    let storage = read_json_file(&storage_path)?;
    
    if let Some(k) = key {
        Ok(storage.get(&k).cloned().unwrap_or(Value::Null))
    } else {
        Ok(storage)
    }
}

#[tauri::command]
pub async fn set_shared_storage(app: AppHandle, key: String, value: Value) -> Result<(), String> {
    let storage_path = get_shared_storage_path(&app)?;
    let mut storage = read_json_file(&storage_path)?;
    
    if let Value::Object(ref mut map) = storage {
        map.insert(key, value);
    }
    
    write_json_file(&storage_path, &storage)
}
