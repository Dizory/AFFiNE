use tauri::{AppHandle, Manager, State, WebviewWindow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowState {
    pub fullscreen: bool,
    pub maximized: bool,
    pub focused: bool,
    pub visible: bool,
}

#[tauri::command]
pub async fn show_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
pub async fn hide_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
pub async fn toggle_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().map_err(|e| e.to_string())? {
            window.hide().map_err(|e| e.to_string())?;
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        }
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
pub async fn get_window_state(app: AppHandle) -> Result<WindowState, String> {
    if let Some(window) = app.get_webview_window("main") {
        Ok(WindowState {
            fullscreen: window.is_fullscreen().map_err(|e| e.to_string())?,
            maximized: window.is_maximized().map_err(|e| e.to_string())?,
            focused: window.is_focused().map_err(|e| e.to_string())?,
            visible: window.is_visible().map_err(|e| e.to_string())?,
        })
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
pub async fn set_window_state(
    app: AppHandle,
    fullscreen: Option<bool>,
    maximized: Option<bool>,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if let Some(fs) = fullscreen {
            window.set_fullscreen(fs).map_err(|e| e.to_string())?;
        }
        
        if let Some(max) = maximized {
            if max {
                window.maximize().map_err(|e| e.to_string())?;
            } else {
                window.unmaximize().map_err(|e| e.to_string())?;
            }
        }
        
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindInPageOptions {
    pub forward: bool,
    pub find_next: bool,
    pub match_case: bool,
}

#[tauri::command]
pub async fn find_in_page(
    window: WebviewWindow,
    text: String,
    options: Option<FindInPageOptions>,
) -> Result<(), String> {
    // Note: Tauri doesn't have built-in find-in-page
    // We need to implement it using webview.eval or custom JS
    let opts = options.unwrap_or(FindInPageOptions {
        forward: true,
        find_next: false,
        match_case: false,
    });
    
    let script = format!(
        r#"
        (function() {{
            if (!window.affine) window.affine = {{}};
            if (!window.affine.findInPage) {{
                window.affine.findInPage = function(text, forward, matchCase) {{
                    if (window.find) {{
                        return window.find(text, matchCase, !forward);
                    }}
                    return false;
                }};
            }}
            return window.affine.findInPage({:?}, {}, {});
        }})()
        "#,
        text, opts.forward, opts.match_case
    );
    
    window.eval(&script).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn stop_find_in_page(window: WebviewWindow) -> Result<(), String> {
    let script = r#"
        (function() {
            if (window.getSelection) {
                window.getSelection().removeAllRanges();
            }
        })()
    "#;
    
    window.eval(script).map_err(|e| e.to_string())?;
    Ok(())
}
