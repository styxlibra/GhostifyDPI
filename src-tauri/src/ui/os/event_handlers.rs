use tauri::{App, AppHandle, CloseRequestApi, Manager, RunEvent, Window, WindowEvent};

use crate::ui::frontend::api;

fn on_window_close(window: &Window, api: &CloseRequestApi) {
    window.hide().unwrap();
    api.prevent_close();
}

pub fn on_window_event(window: &Window, event: &WindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event {
        on_window_close(window, api);
    }
}

fn on_application_start(app_handle: &AppHandle) {
    api::show_window(app_handle.clone());
}

pub fn on_runtime_event(app_handle: &AppHandle, event: RunEvent) {
    if let RunEvent::Ready = event {} else { return; }
    on_application_start(app_handle);
}

pub fn setup_signal_handler(app: &mut App) -> Result<(), ctrlc::Error> {
    let app_handle = app.app_handle().clone();
    ctrlc::set_handler(move || {
        api::exit_app(app_handle.clone());
    })
}
