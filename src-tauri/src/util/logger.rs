use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{LazyLock, Mutex, OnceLock};

use crate::util::current_exe_dir;

static MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
static LOG_FILE: OnceLock<File> = OnceLock::new();

fn get_log_file() -> &'static File {
    if let Some(log_file) = LOG_FILE.get() { return log_file; }
    let log_path = current_exe_dir().join("GhostifyDPI.log");
    let log_file = OpenOptions::new().append(true).create(true).open(log_path).unwrap();
    LOG_FILE.set(log_file).unwrap();
    LOG_FILE.get().unwrap()
}

pub fn log(message: impl AsRef<OsStr>) {
    let _synchronized = MUTEX.lock().unwrap();
    let mut log_file = get_log_file();
    let msg = message.as_ref().to_str().unwrap();
    println!("{}", msg);
    let line = if msg.ends_with("\r\n") { msg.to_owned() }
        else if msg.ends_with("\n") { msg.replace("\n", "\r\n") }
        else if msg.ends_with("\r") { msg.to_owned() + "\n" }
        else { msg.to_owned() + "\r\n" };
    log_file.write(line.as_bytes()).unwrap();
    log_file.flush().unwrap();
}
