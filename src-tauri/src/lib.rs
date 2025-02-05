#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod process;
mod ui;
mod util;

use tauri::{App, Manager};

use crate::process::launcher;
use crate::ui::frontend::{api, events};
use crate::ui::os::{event_handlers, tray};

fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    event_handlers::setup_signal_handler(app)?;
    tray::setup(app)?;
    events::setup_status_emitter(app.app_handle().clone());
    Ok(())
}

pub fn run() {
    launcher::initialize();
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_positioner::init())
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            api::start,
            api::stop,
            api::status,
            api::open_url,
        ])
        .on_window_event(event_handlers::on_window_event)
        .build(tauri::generate_context!())
        .expect("Error while running Ghostify DPI");
    app.run(event_handlers::on_runtime_event);
}
