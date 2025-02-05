use std::str::FromStr;
use std::sync::{Arc, LazyLock, Mutex};
use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuEvent, MenuItem, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Error, Event, Listener, Manager, Theme, WebviewWindow, Wry};

use crate::process::launch_stages::LaunchStages;
use crate::ui::frontend::api;
use crate::ui::os::icons::ICONS;

static TRAY_ID: LazyLock<Arc<Mutex<Option<String>>>> = LazyLock::new(|| Arc::new(Mutex::new(None)));

fn get_window(app: &App) -> WebviewWindow {
    app.get_webview_window("main").unwrap()
}

fn get_tray_id() -> String {
    let tray_id = TRAY_ID.lock().unwrap();
    tray_id.as_ref().unwrap().to_string()
}

fn get_tray(app: &App) -> TrayIcon {
    app.tray_by_id(&get_tray_id()).unwrap()
}

fn get_status(event: Event) -> String {
    event.payload().to_owned().replace("\"", "")
}

fn get_tooltip(status: String) -> String {
    format!("Ghostify DPI\r\nStatus: {}", status)
}

fn setup_status_item_updater(app: &App, start_item: MenuItem<Wry>, stop_item: MenuItem<Wry>) {
    app.listen_any("status", move |event| {
        let status = LaunchStages::from_str(get_status(event).as_str()).unwrap();
        start_item.set_enabled(status == LaunchStages::Stopped).unwrap();
        stop_item.set_enabled(status == LaunchStages::Started).unwrap();
    });
}

fn setup_tooltip_updater(app: &App) {
    let tray = get_tray(app);
    app.listen_any("status", move |event| {
        let tooltip = get_tooltip(get_status(event));
        tray.set_tooltip(Some(tooltip)).unwrap_or_default();
    });
}

fn get_icon(window: &WebviewWindow) -> Image {
    let scale = window.scale_factor().unwrap();
    let size = match scale {
        1.0 => 16,
        1.25 => 20,
        1.5 => 24,
        2.0 => 32,
        2.25 => 36,
        2.5 => 40,
        3.0 => 48,
        4.0 => 64,
        4.5 => 72,
        5.0 => 80,
        _ => 256,
    };
    let icon_set = ICONS.get_key_value(&size).unwrap().1;
    let theme = window.theme().unwrap();
    if theme == Theme::Light { icon_set.icon_light() } else { icon_set.icon_dark() }
}

fn setup_icon_updater(app: &App, event_name: String) {
    let window = get_window(app);
    let tray = get_tray(app);
    app.listen_any(event_name, move |_event| {
        tray.set_icon(Some(get_icon(&window))).unwrap_or_default();
    });
}

fn on_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    let app_handle = tray.app_handle();
    tauri_plugin_positioner::on_tray_event(app_handle, &event);
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event {} else { return; }
    api::show_window(app_handle.clone());
}

fn on_menu_event(app_handle: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "start" => api::start(),
        "stop" => api::stop(),
        "hide" => api::hide_window(app_handle.clone()),
        "quit" => api::exit_app(app_handle.clone()),
        _ => {}
    }
}

pub fn setup(app: &App) -> Result<TrayIcon, Error> {
    let start = MenuItemBuilder::with_id("start", "Start").build(app)?;
    let stop = MenuItemBuilder::with_id("stop", "Stop").enabled(false).build(app)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let hide = MenuItemBuilder::with_id("hide", "Hide").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
    let menu = MenuBuilder::new(app)
        .item(&start)
        .item(&stop)
        .item(&separator)
        .item(&hide)
        .item(&quit)
        .build()?;
    let window = get_window(app);
    let icon = get_icon(&window);
    let tooltip = get_tooltip(LaunchStages::Stopped.to_string());
    let tray = TrayIconBuilder::new()
        .icon(icon)
        .tooltip(tooltip)
        .menu(&menu)
        .icon_as_template(false)
        .on_tray_icon_event(on_icon_event)
        .on_menu_event(on_menu_event)
        .build(app)?;
    *TRAY_ID.lock().unwrap() = Some(tray.id().0.clone());
    setup_icon_updater(app, "tauri://scale-change".to_string());
    setup_icon_updater(app, "tauri://theme-changed".to_string());
    setup_tooltip_updater(app);
    setup_status_item_updater(app, start, stop);
    Ok(tray)
}

pub fn hide_icon(app: AppHandle) {
    let tray_id = get_tray_id();
    app.tray_by_id(&tray_id).unwrap().set_visible(false).unwrap();
    app.remove_tray_by_id(&tray_id).unwrap();
}
