use tauri::{
    App, AppHandle, Manager,
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
};

pub fn setup_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let quit_item = MenuItem::with_id(app, "quit", "Quit AFFiNE", true, None::<&str>)?;
    let show_item = MenuItem::with_id(app, "show", "Show AFFiNE", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "Hide AFFiNE", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    
    let menu = Menu::with_items(
        app,
        &[
            &show_item,
            &hide_item,
            &separator,
            &quit_item,
        ]
    )?;
    
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "quit" => {
                    app.exit(0);
                }
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "hide" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { .. } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;
    
    Ok(())
}
