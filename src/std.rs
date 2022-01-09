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
    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute diskutil");

    let (sender, receiver) = channel();

    // Spawn off an expensive computation
    let handle = thread::spawn(move || {
        let stdout = child
            .stdout
            .take()
            .expect("child did not have a handle to stdout");

        let mut stdout_reader = BufReader::new(stdout);

        let stderr = child
            .stderr
            .take()
            .expect("child did not have a handle to stderr");

        let mut stderr_reader = BufReader::new(stderr);

        loop {
            match action.recv_timeout(Duration::from_millis(100)) {
                Ok(action) => match action {
                    Action::Stop => {
                        eprintln!("<stop>");
                        sender.send(None).unwrap_or(());
                        return child.kill().map_err(|e| Error::from(e));
                    }
                    Action::Noop => {
                        eprintln!("<noop>");
                    }
                },
                Err(e) => {
                    eprintln!("<thread> {}", e);
                    return child.kill().map_err(|e| Error::from(e));
                }
            }
            let mut outbuf: Vec<u8> = Vec::new();
            if let Ok(_bytes_read) = stdout_reader.read_until(b'\n', &mut outbuf) {
                let line = String::from_utf8(outbuf).unwrap();
                if line.starts_with("***Begin monitoring") {
                    continue;
                }
                let event = Event::from_line(line.as_str());
                sender.send(Some(event)).unwrap();
            }
            match child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) => continue,
                Err(e) => {
                    eprintln!("failed to read output of diskutil activity: {}", e);
                    break;
                }
            }
        }
        sender.send(None).unwrap();
        Ok(())
    });

    (handle, receiver)
}

pub fn stream_events(
    action: Receiver<Action>,
) -> (
    thread::JoinHandle<Result<(), Error>>,
    Receiver<Option<Event>>,
) {
    stream_events_with_command("/usr/sbin/diskutil", vec!["activity"], action)
}
