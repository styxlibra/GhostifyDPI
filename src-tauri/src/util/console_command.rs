use async_channel::{Receiver, Sender};
use futures_lite::future::block_on;
use portable_pty::{native_pty_system, ChildKiller, CommandBuilder, ExitStatus, PtyPair, PtySize};
use std::ffi::OsStr;
use std::io::{Error, Read};

type OnOutputCallback = dyn Fn(ConsoleReaderWrapper) -> ();
type OnHandleCallback = dyn Fn(ConsoleProcess) -> ();

pub struct ConsoleProcess {
    pub id: u8,
    handle: Box<dyn ChildKiller + Send + Sync>,
    kill_channel: Sender<bool>,
}

impl ConsoleProcess {
    pub fn kill(&mut self) -> Result<(), Error> {
        let result = self.handle.kill();
        block_on(async {
            self.kill_channel.send(true).await.unwrap_or(());
        });
        result
    }
}

pub struct ConsoleReaderWrapper {
    pub process_id: u8,
    pair: PtyPair,
    pub process_kill_channel: Receiver<bool>,
}

impl ConsoleReaderWrapper {
    pub fn extract(&self) -> Box<dyn Read + Send> {
        self.pair.master.try_clone_reader().unwrap()
    }
}

pub struct ConsoleCommand<'a> {
    cmd: CommandBuilder,
    process_id: u8,
    on_output_callback: Option<&'a OnOutputCallback>,
    on_handle_callback: Option<&'a OnHandleCallback>,
}

impl<'a> ConsoleCommand<'a> {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            cmd: CommandBuilder::new(program),
            process_id: 0,
            on_output_callback: None,
            on_handle_callback: None,
        }
    }

    pub fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.cmd.arg(arg);
        self
    }

    pub fn with_process_id(&mut self, process_id: u8) -> &mut Self {
        self.process_id = process_id;
        self
    }

    pub fn on_output(&mut self, callback: &'a OnOutputCallback) -> &mut Self {
        self.on_output_callback = Some(callback);
        self
    }

    pub fn on_handle(&mut self, callback: &'a OnHandleCallback) -> &mut Self {
        self.on_handle_callback = Some(callback);
        self
    }

    pub fn spawn(&self) -> ExitStatus {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize::default()).unwrap();
        let cmd = self.cmd.clone();
        let mut process = pair.slave.spawn_command(cmd).unwrap();
        {
            let writer = pair.master.take_writer().unwrap();
            drop(writer);
        }
        let (sender, receiver) = async_channel::unbounded::<bool>();
        if let Some(callback) = &self.on_output_callback {
            let reader_wrapper = ConsoleReaderWrapper {
                process_id: self.process_id,
                pair,
                process_kill_channel: receiver,
            };
            callback(reader_wrapper);
        } else {
            drop(receiver);
        }
        if let Some(callback) = &self.on_handle_callback {
            let console_process = ConsoleProcess {
                id: self.process_id,
                handle: process.clone_killer(),
                kill_channel: sender,
            };
            callback(console_process);
        } else {
            drop(sender);
        }
        process.wait().unwrap()
    }
}
