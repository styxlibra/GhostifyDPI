use std::ffi::OsStr;
use std::process::Command;
use std::str::from_utf8;

#[cfg(runtime)]
use crate::util::logger;

pub const ICON_SIZES: [u16; 14] = [16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 256];

pub fn log(message: impl AsRef<OsStr>) {
    #[cfg(runtime)]
    logger::log(message);
    #[cfg(not(runtime))]
    println!("cargo:warning={}", message.as_ref().to_str().unwrap());
}

pub fn execute(command: Vec<&str>) {
    log(command.join(" "));
    let mut cmd = Command::new(command[0]);
    for i in 1..command.len() {
        cmd.arg(command[i]);
    }
    let output = cmd.output().expect("Failed to execute command");
    let message = from_utf8(&output.stdout).unwrap();
    log(message);
}

pub fn kill_goodbyedpi() {
    execute(vec!["taskkill.exe", "/f", "/t", "/im", "goodbyedpi.exe"]);
    execute(vec!["sc.exe", "stop", "windivert14"]);
    execute(vec!["sc.exe", "delete", "windivert14"]);
    execute(vec!["sc.exe", "stop", "windivert"]);
    execute(vec!["sc.exe", "delete", "windivert"]);
}
