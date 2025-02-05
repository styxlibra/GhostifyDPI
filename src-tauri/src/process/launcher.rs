use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, LazyLock, Mutex};
use std::time::Duration;
use async_std::task::sleep;
use bitness::{os_bitness, Bitness};
use blocking::Unblock;
use futures_lite::future::block_on;
use futures_lite::io::BufReader;
use futures_lite::{AsyncBufReadExt, FutureExt, StreamExt};

use crate::process::launch_stages::LaunchStages;
use crate::process::process_status::{get_status, set_status, switch_status};
use crate::util::console_command::{ConsoleCommand, ConsoleProcess, ConsoleReaderWrapper};
use crate::util::logger;
use crate::util::{current_exe_dir, kill_goodbyedpi};

static PROCESS_ID: AtomicU8 = AtomicU8::new(0);
static PROCESS: LazyLock<Arc<Mutex<Option<ConsoleProcess>>>> = LazyLock::new(|| Arc::new(Mutex::new(None)));
static PROCESS_START_CHANNEL: LazyLock<Arc<Mutex<Option<Sender<bool>>>>> = LazyLock::new(|| Arc::new(Mutex::new(None)));
static PROCESS_READY_SENDER: LazyLock<Arc<Mutex<Option<async_channel::Sender<u32>>>>> = LazyLock::new(|| Arc::new(Mutex::new(None)));
static PROCESS_READY_RECEIVER: LazyLock<Arc<Mutex<Option<async_channel::Receiver<u32>>>>> = LazyLock::new(|| Arc::new(Mutex::new(None)));

pub fn initialize() {
    initialize_ready_channel();
    initialize_start_channel();
}

fn initialize_ready_channel() {
    let (sender, receiver) = async_channel::unbounded::<u32>();
    *PROCESS_READY_SENDER.lock().unwrap() = Some(sender);
    *PROCESS_READY_RECEIVER.lock().unwrap() = Some(receiver);
}

fn initialize_start_channel() {
    let (sender, receiver) = mpsc::channel::<bool>();
    *PROCESS_START_CHANNEL.lock().unwrap() = Some(sender);
    std::thread::spawn(move || {
        while let Ok( _ ) = receiver.recv() {
            launch_goodbyedpi();
        }
    });
}

fn launch_goodbyedpi() {
    let process_id = PROCESS_ID.fetch_add(1, Ordering::Relaxed);
    let folder = if os_bitness().unwrap()==Bitness::X86_64 { "x86_64" } else { "x86" };
    let executable = current_exe_dir().join(folder).join("goodbyedpi.exe");
    let blacklist = blacklist_file().canonicalize().unwrap();
    let exit_status = ConsoleCommand::new(executable)
        .arg("-9")
        .arg("--blacklist")
        .arg(blacklist)
        .with_process_id(process_id)
        .on_output(&on_output)
        .on_handle(&on_handle)
        .spawn();
    let stage = get_status();
    if stage == LaunchStages::Started as u8 {
        logger::log("goodbyedpi.exe killed");
    }
    logger::log(exit_status.to_string());
    stop_id(process_id);
    if stage == LaunchStages::Starting as u8 {
        let ready_sender = PROCESS_READY_SENDER.lock().unwrap().clone().unwrap();
        ready_sender.send_blocking(exit_status.exit_code()).unwrap_or(());
    }
}

fn on_output(reader_wrapper: ConsoleReaderWrapper) {
    let ready_sender = PROCESS_READY_SENDER.lock().unwrap().clone().unwrap();
    tauri::async_runtime::spawn(async move {
        let mut started = false;
        let process_id = reader_wrapper.process_id;
        let reader = reader_wrapper.extract();
        let receiver = reader_wrapper.process_kill_channel;
        let mut lines = BufReader::new(Unblock::new(reader)).lines();
        loop {
            let wait_timeout = async {
                sleep(Duration::from_secs(5)).await;
                Some(Err(std::io::Error::other("Timed out waiting for process to start")))
            };
            let check_if_killed = async {
                match receiver.recv().await {
                    Ok(_) => Some(Err(std::io::Error::other("Process killed"))),
                    Err(error) => Some(Err(std::io::Error::other(error.to_string()))),
                }
            };
            let future = lines.next().or(check_if_killed);
            let result;
            if started {
                result = future.await;
            } else {
                result = future.or(wait_timeout).await;
            }
            match result {
                Some(Ok(line)) => {
                    let is_success = line.contains("GoodbyeDPI is now running");
                    if is_success {
                        started = true;
                        ready_sender.send(0).await.unwrap_or(());
                    }
                    logger::log(line);
                },
                Some(Err(error)) => {
                    logger::log(format!("Output error: {}", error.to_string()));
                    break;
                },
                None => break,
            }
        }
        logger::log("End of output");
        stop_id(process_id);
    });
}

fn on_handle(process: ConsoleProcess) {
    *PROCESS.lock().unwrap() = Some(process);
}

pub fn status() -> String {
    LaunchStages::from_repr(get_status()).unwrap().to_string()
}

pub fn start() {
    if !switch_status(LaunchStages::Stopped, LaunchStages::Starting) { return; }
    create_default_blacklist();
    kill_goodbyedpi();
    PROCESS_START_CHANNEL.lock().unwrap().clone().unwrap().send(true).unwrap();
    let exit_code = block_on(async {
        PROCESS_READY_RECEIVER.lock().unwrap().clone().unwrap().recv().await.unwrap()
    });
    std::thread::sleep(Duration::from_millis(1000));
    let stage = if exit_code == 0 { LaunchStages::Started } else { LaunchStages::Stopped };
    set_status(stage);
}

fn create_default_blacklist() {
    if blacklist_file().exists() { return; }
    let default_list = include_bytes!("./default_blacklist.txt");
    // TODO: show alert
    fs::write(blacklist_file(), default_list)
        .expect("Couldn't write blacklist.txt");
}

fn blacklist_file() -> PathBuf {
    current_exe_dir().join("blacklist.txt")
}

pub fn stop() {
    let process_id = if let Some( process ) = PROCESS.lock().unwrap().as_ref() { Some(process.id) } else { None };
    if let Some( id ) = process_id {
        stop_id(id);
    }
}

fn stop_id(process_id: u8) {
    if !switch_status(LaunchStages::Started, LaunchStages::Stopping) { return; }
    let is_same = if let Some( process ) = PROCESS.lock().unwrap().as_ref() { process.id == process_id } else { true };
    if !is_same { return set_status(LaunchStages::Started); }
    if let Some( mut process ) = PROCESS.lock().unwrap().take() {
        if let Err( error ) = process.kill() {
            logger::log(format!("Error on stop: {}", error.to_string()));
        }
        logger::log("goodbyedpi.exe stopped");
    }
    std::thread::sleep(Duration::from_millis(1000));
    kill_goodbyedpi();
    set_status(LaunchStages::Stopped);
}
