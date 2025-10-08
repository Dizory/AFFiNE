/**
 * Integration with AFFiNE native Rust modules
 * 
 * This module provides bridges to:
 * - affine_common: Common utilities and types
 * - affine_nbstore: Local-first database storage
 */

use affine_nbstore::{BlobStorage, DocStorage};
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct NativeStorage {
    storage_path: PathBuf,
    blob_storage: Arc<RwLock<Option<BlobStorage>>>,
    doc_storage: Arc<RwLock<Option<DocStorage>>>,
}

impl NativeStorage {
    pub fn new(storage_path: PathBuf) -> Self {
        Self {
            storage_path,
            blob_storage: Arc::new(RwLock::new(None)),
            doc_storage: Arc::new(RwLock::new(None)),
        }
    }
    
    pub async fn init(&self) -> Result<(), String> {
        // Initialize blob storage
        let blob_storage = BlobStorage::new(&self.storage_path.join("blobs"))
            .map_err(|e| format!("Failed to initialize blob storage: {}", e))?;
        
        *self.blob_storage.write() = Some(blob_storage);
        
        // Initialize doc storage
        let doc_storage = DocStorage::new(&self.storage_path.join("docs"))
            .map_err(|e| format!("Failed to initialize doc storage: {}", e))?;
        
        *self.doc_storage.write() = Some(doc_storage);
        
        log::info!("Native storage initialized at {:?}", self.storage_path);
        
        Ok(())
    }
    
    pub fn get_blob_storage(&self) -> Option<BlobStorage> {
        self.blob_storage.read().clone()
    }
    
    pub fn get_doc_storage(&self) -> Option<DocStorage> {
        self.doc_storage.read().clone()
    }
}

// Tauri commands for native storage operations

#[tauri::command]
pub async fn native_storage_get_blob(
    storage: tauri::State<'_, NativeStorage>,
    key: String,
) -> Result<Vec<u8>, String> {
    let blob_storage = storage.get_blob_storage()
        .ok_or_else(|| "Blob storage not initialized".to_string())?;
    
    blob_storage.get(&key)
        .map_err(|e| format!("Failed to get blob: {}", e))
}

#[tauri::command]
pub async fn native_storage_set_blob(
    storage: tauri::State<'_, NativeStorage>,
    key: String,
    data: Vec<u8>,
) -> Result<(), String> {
    let blob_storage = storage.get_blob_storage()
        .ok_or_else(|| "Blob storage not initialized".to_string())?;
    
    blob_storage.set(&key, &data)
        .map_err(|e| format!("Failed to set blob: {}", e))
}

#[tauri::command]
pub async fn native_storage_delete_blob(
    storage: tauri::State<'_, NativeStorage>,
    key: String,
) -> Result<(), String> {
    let blob_storage = storage.get_blob_storage()
        .ok_or_else(|| "Blob storage not initialized".to_string())?;
    
    blob_storage.delete(&key)
        .map_err(|e| format!("Failed to delete blob: {}", e))
}

#[tauri::command]
pub async fn native_storage_get_doc(
    storage: tauri::State<'_, NativeStorage>,
    doc_id: String,
) -> Result<Vec<u8>, String> {
    let doc_storage = storage.get_doc_storage()
        .ok_or_else(|| "Doc storage not initialized".to_string())?;
    
    doc_storage.get(&doc_id)
        .map_err(|e| format!("Failed to get doc: {}", e))
}

#[tauri::command]
pub async fn native_storage_set_doc(
    storage: tauri::State<'_, NativeStorage>,
    doc_id: String,
    data: Vec<u8>,
) -> Result<(), String> {
    let doc_storage = storage.get_doc_storage()
        .ok_or_else(|| "Doc storage not initialized".to_string())?;
    
    doc_storage.set(&doc_id, &data)
        .map_err(|e| format!("Failed to set doc: {}", e))
}

#[tauri::command]
pub async fn native_storage_delete_doc(
    storage: tauri::State<'_, NativeStorage>,
    doc_id: String,
) -> Result<(), String> {
    let doc_storage = storage.get_doc_storage()
        .ok_or_else(|| "Doc storage not initialized".to_string())?;
    
    doc_storage.delete(&doc_id)
        .map_err(|e| format!("Failed to delete doc: {}", e))
}
