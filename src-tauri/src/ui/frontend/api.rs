use tauri::{command, AppHandle, Manager};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::process::launcher;
use crate::ui::os::tray;
use crate::util;

pub fn exit_app(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        tray::hide_icon(app_handle.clone());
        launcher::stop();
        app_handle.exit(0);
    });
}

pub fn show_window(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    if !window.is_visible().unwrap() {
        window.move_window(Position::Center).unwrap();
    }
    window.show().unwrap();
    window.set_focus().unwrap();
}

pub fn hide_window(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    window.hide().unwrap();
}

#[command]
pub fn start() {
    tauri::async_runtime::spawn(async move {
        launcher::start();
    });
}

#[command]
pub fn stop() {
    tauri::async_runtime::spawn(async move {
        launcher::stop();
    });
}

#[command]
pub fn status() -> String {
    launcher::status()
}

#[command]
pub fn open_url(url: String) {
    util::execute(vec!["cmd.exe", "/C", "start", url.as_str()]);
}
