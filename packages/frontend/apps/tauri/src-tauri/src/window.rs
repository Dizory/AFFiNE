use tauri::{Manager, WebviewWindow, WebviewWindowBuilder};
use tauri::menu::{Menu, MenuItem};

pub fn create_main_window(app: &tauri::AppHandle) -> Result<WebviewWindow, tauri::Error> {
    let window = WebviewWindowBuilder::new(
        app,
        "main",
        tauri::WebviewUrl::default()
    )
    .title("AFFiNE")
    .inner_size(1000.0, 800.0)
    .min_inner_size(640.0, 480.0)
    .resizable(true)
    .fullscreen(false)
    .decorations(true)
    .transparent(false)
    .center()
    .build()?;
    
    Ok(window)
}

pub fn create_popup_window(
    app: &tauri::AppHandle,
    label: &str,
    url: String,
    width: f64,
    height: f64,
) -> Result<WebviewWindow, tauri::Error> {
    let window = WebviewWindowBuilder::new(
        app,
        label,
        tauri::WebviewUrl::External(url.parse().unwrap())
    )
    .title("AFFiNE Popup")
    .inner_size(width, height)
    .resizable(true)
    .fullscreen(false)
    .decorations(true)
    .center()
    .build()?;
    
    Ok(window)
}
