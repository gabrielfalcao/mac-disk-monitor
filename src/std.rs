#![allow(rustdoc::bare_urls)]

use crate::event::Event;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::SendError;
use std::time::Duration;
use thiserror;
use timeout_readwrite::TimeoutReader;

use std::sync::mpsc::{channel, Receiver};
use std::thread;

/// The error type for this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("utf-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("thread send error: {0}")]
    SendError(#[from] SendError<Option<Event>>),
}

/// The Action that can be sent to the thread to stop it
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Stop,
    Noop,
}

/// Runs `diskutil activity` in a thread and parses its stdout in real
/// time, emitting Option<Event> when necessary.
///
/// Parameters:
/// > `action` - a [`Receiver`] where [`Action`] can be sent to the thread.
/// [`Receiver`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
#[cfg(not(tarpaulin_include))] // TODO find a way to write a test for this
pub fn stream_events(
    action: Receiver<Action>,
) -> (
    thread::JoinHandle<Result<(), Error>>,
    Receiver<Option<Event>>,
) {
    stream_events_with_command("/usr/sbin/diskutil", vec!["activity"], action)
}

/// Runs the given command in a thread and attempts to parse event
/// data from each new line of the subprocess's stdout.
/// This is the underlying function that does all the heavy lifting for [`stream_events`].
///
/// Parameters:
/// > `command` - the command to execute
/// > `args` - the command-line args to pass to the command
/// > `action` - a [`Receiver`] where [`Action`] can be sent to the thread.
/// [`Receiver`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
/// [`stream_events`]: ./fn.stream_events.html
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

        let mut stdout_reader =
            BufReader::new(TimeoutReader::new(stdout, Duration::from_millis(314)));

        loop {
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
                Ok(None) => match action.recv_timeout(Duration::from_millis(100)) {
                    Ok(action) => match action {
                        Action::Stop => {
                            return child.kill().map_err(|e| Error::from(e));
                        }
                        Action::Noop => continue,
                    },
                    Err(_) => {}
                },
                Err(_) => {
                    break;
                }
            }
        }
        Ok(())
    });

    (handle, receiver)
}
