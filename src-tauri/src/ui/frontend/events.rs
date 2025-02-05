use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, LazyLock, Mutex};

use tauri::{AppHandle, Emitter};

use crate::process::launch_stages::LaunchStages;

static STATUS_CHANNEL: LazyLock<Arc<Mutex<Option<Sender<LaunchStages>>>>> = LazyLock::new(|| Arc::new(Mutex::new(None)));

pub fn setup_status_emitter(app_handle: AppHandle) {
    let (sender, receiver) = mpsc::channel::<LaunchStages>();
    let mut channel = STATUS_CHANNEL.lock().unwrap();
    if let Some( _ ) = *channel {
        return;
    }
    *channel = Some(sender);
    std::thread::spawn(move || {
        while let Ok( status ) = receiver.recv() {
            app_handle.emit("status", status.to_string()).unwrap();
        }
    });
}

pub fn emit_status(status: LaunchStages) {
    STATUS_CHANNEL.lock().unwrap().clone().unwrap().send(status).unwrap();
}
