#![allow(unused)]
use k9::assert_equal;
use mac_disk_monitor::event::Event;
use mac_disk_monitor::std::*;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

#[test]
fn test_disk_activity() {
    let (action_sender, action_receiver) = channel();
    let (thread, receiver) =
        stream_events_with_command("./tests/dummy-disk-activity.sh", vec![], action_receiver);

    let event = receiver
        .recv_timeout(Duration::from_secs(5))
        .unwrap_or(None);
    assert_ne!(event, None);
    let event = event.unwrap();
    assert_equal!(event.name(), "DiskAppeared");
    assert_equal!(
        event.path().unwrap_or(String::new()),
        "file:///Volumes/my%20backups/"
    );

    action_sender.send(Action::Stop).unwrap();
    thread.join().unwrap().unwrap();
}
