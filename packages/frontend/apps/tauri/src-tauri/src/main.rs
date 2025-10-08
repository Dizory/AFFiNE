// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;
mod window;
mod updater;
mod tray;
mod clipboard;
mod storage;
mod recording;
mod ui;
mod native_integration;

use tauri::{Manager, WindowEvent};
use tauri_plugin_deep_link;
use tauri_plugin_single_instance;

fn main() {
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            log::info!("Single instance callback: {:?}, {:?}", argv, cwd);
            
            // Handle deep link from second instance
            if let Some(url) = argv.last() {
                if url.starts_with("affine://") || url.starts_with("affine-") {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.set_focus();
                        let _ = window.emit("deep-link", url);
                    }
                }
            }
        }))
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            // Initialize application state
            state::init_app_state(app)?;
            
            // Initialize native storage
            let storage_path = app.path().app_data_dir()?;
            let native_storage = native_integration::NativeStorage::new(storage_path);
            
            // Initialize storage asynchronously
            let storage_clone = native_storage.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = storage_clone.init().await {
                    log::error!("Failed to initialize native storage: {}", e);
                }
            });
            
            app.manage(native_storage);
            
            // Setup deep link handler
            tauri_plugin_deep_link::register("affine", move |request| {
                log::info!("Deep link received: {}", request);
            })?;
            
            // Register custom protocol for affine://
            let handle = app.handle().clone();
            tauri_plugin_deep_link::register_all(&["affine", "affine-canary", "affine-beta", "affine-internal", "affine-dev"], move |request| {
                log::info!("Protocol request: {}", request);
                if let Some(window) = handle.get_webview_window("main") {
                    let _ = window.emit("protocol-request", request);
                }
            })?;
            
            // Setup tray menu
            tray::setup_tray(app)?;
            
            // Setup main window
            let window = app.get_webview_window("main").unwrap();
            window.on_window_event(|event| {
                match event {
                    WindowEvent::CloseRequested { api, .. } => {
                        #[cfg(target_os = "macos")]
                        {
                            // On macOS, hide instead of close
                            api.prevent_close();
                            event.window().hide().unwrap();
                        }
                    }
                    _ => {}
                }
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Debug commands
            commands::debug::reveal_log_file,
            commands::debug::get_log_file_path,
            
            // UI commands
            commands::ui::show_main_window,
            commands::ui::hide_main_window,
            commands::ui::toggle_window,
            commands::ui::get_window_state,
            commands::ui::set_window_state,
            
            // Clipboard commands
            commands::clipboard::read_text,
            commands::clipboard::write_text,
            commands::clipboard::read_image,
            commands::clipboard::write_image,
            
            // Storage commands
            commands::storage::get_config,
            commands::storage::set_config,
            commands::storage::delete_config,
            commands::storage::get_shared_storage,
            commands::storage::set_shared_storage,
            
            // Updater commands
            commands::updater::check_for_updates,
            commands::updater::download_update,
            commands::updater::install_update,
            commands::updater::get_current_version,
            commands::updater::get_updater_config,
            commands::updater::set_updater_config,
            
            // Recording commands (if supported)
            commands::recording::get_recording_status,
            commands::recording::start_recording,
            commands::recording::stop_recording,
            commands::recording::get_recordings,
            
            // Find in page
            commands::ui::find_in_page,
            commands::ui::stop_find_in_page,
            
            // I18n
            commands::i18n::change_language,
            
            // Worker
            commands::worker::spawn_worker,
            commands::worker::terminate_worker,
            
            // Native storage
            native_integration::native_storage_get_blob,
            native_integration::native_storage_set_blob,
            native_integration::native_storage_delete_blob,
            native_integration::native_storage_get_doc,
            native_integration::native_storage_set_doc,
            native_integration::native_storage_delete_doc,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
