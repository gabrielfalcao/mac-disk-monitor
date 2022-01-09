#![allow(unused)]
use crate::event::Event;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::SendError;
use std::time::Duration;
use thiserror;

use std::sync::mpsc::{channel, Receiver};
use std::thread;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("utf-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("thread send error: {0}")]
    SendError(#[from] SendError<Option<Event>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Stop,
    Noop,
}
pub fn stream_events_with_command(
    command: &str,
    args: Vec<&str>,
    action: Receiver<Action>,
) -> (
    thread::JoinHandle<Result<(), Error>>,
    Receiver<Option<Event>>,
) {
    let (sender, receiver) = channel();

    let handle = thread::spawn(move || {
        let event = Event::from_line("***DiskDescriptionChanged ('disk4', DAVolumePath = 'file:///Volumes/my%20backups/') Time=20220108-20:26:52.7814");
        thread::sleep(Duration::from_secs(2));
        sender.send(Some(event)).unwrap();
        match action.recv_timeout(Duration::from_secs(1)) {
            Ok(action) => {
                sender.send(None).unwrap();
                return Ok(());
            }
            Err(_) => {
                thread::sleep(Duration::from_secs(1));
            }
        }
        sender.send(None).unwrap();

        Ok(())
    });

    (handle, receiver)
}
