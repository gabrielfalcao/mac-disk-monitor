extern crate clap;
use clap::App;

use mac_disk_monitor::std::*;
use std::sync::mpsc::channel;
use std::time::Duration;

/// Command-line entrypoint to monitor the disk events and print them out to stdout as json.
#[cfg(not(tarpaulin_include))]
fn main() {
    let app = App::new("mac-disk-monitor");
    let _matches = app.get_matches();

    let (action, receiver) = channel();
    let (_thread, receiver) = stream_events(receiver);

    loop {
        match receiver.recv_timeout(Duration::from_millis(3145)) {
            Ok(event) => match event {
                Some(event) => {
                    println!("{}", event.to_json());
                }
                None => {
                    action.send(Action::Stop).unwrap();
                }
            },
            Err(e) => {
                if !e.to_string().eq("timed out waiting on channel") {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
