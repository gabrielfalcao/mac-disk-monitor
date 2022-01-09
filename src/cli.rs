extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use mac_disk_monitor::event::Event;
use mac_disk_monitor::std::*;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

fn main() {
    let app = App::new("mac-disk-monitor");
    let matches = app.get_matches();

    let (action_sender, action_receiver) = channel();
    let (thread, receiver) =
        stream_events_with_command("/usr/sbin/diskutil", vec!["activity"], action_receiver);

    loop {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                println!("{:?}", event);
            }
        }
    }
}
