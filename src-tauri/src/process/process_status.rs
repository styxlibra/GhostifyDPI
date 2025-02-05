use std::sync::atomic::{AtomicU8, Ordering};

use crate::process::launch_stages::LaunchStages;
use crate::ui::frontend::events::emit_status;

static LAUNCH_STAGE: AtomicU8 = AtomicU8::new(LaunchStages::Stopped as u8);

pub fn get_status() -> u8 {
    LAUNCH_STAGE.load(Ordering::Relaxed)
}

pub fn set_status(stage: LaunchStages) {
    LAUNCH_STAGE.store(stage as u8, Ordering::Relaxed);
    emit_status(stage);
}

pub fn switch_status(current: LaunchStages, new: LaunchStages) -> bool {
    if let Err(_old) = LAUNCH_STAGE.compare_exchange(
        current as u8,
        new as u8,
        Ordering::Relaxed,
        Ordering::Relaxed,
    ) { return false; }
    emit_status(new);
    return true;
}
