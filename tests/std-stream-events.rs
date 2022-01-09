#![allow(unused)]
use k9::assert_equal;
use mac_disk_monitor::event::Event;
use mac_disk_monitor::std::{stream_events, Action};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

#[test]
fn test_disk_activity() {
    let (action_sender, action_receiver) = channel();
    let (thread, receiver) = stream_events(action_receiver);

    let event = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
    assert_ne!(event, None);
    let event = event.unwrap();
    assert_ne!(event.name(), "");
    action_sender.send(Action::Stop).unwrap();
    thread.join().unwrap().unwrap();
}
