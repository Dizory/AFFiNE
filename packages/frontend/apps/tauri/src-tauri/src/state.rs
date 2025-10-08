use tauri::{App, Manager};
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub window_visible: bool,
    pub recording_active: bool,
    pub language: String,
}

pub struct GlobalState {
    pub app_state: Arc<RwLock<AppState>>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            app_state: Arc::new(RwLock::new(AppState {
                window_visible: true,
                recording_active: false,
                language: "en".to_string(),
            })),
        }
    }
}

pub fn init_app_state(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let state = GlobalState::default();
    app.manage(state);
    
    log::info!("Application state initialized");
    
    Ok(())
}
