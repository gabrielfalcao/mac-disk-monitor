use mac_disk_monitor::{stream_events, Action};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let (action, receiver) = channel();
    let (thread, receiver) = stream_events(receiver);

    loop {
        // poll for events every 1.5 seconds
        match receiver.recv_timeout(Duration::from_millis(1500)) {
            Ok(event) => match event {
                Some(event) => {
                    println!("{}", event.to_json());
                }
                None => {
                    eprintln!("thread has stopped...");
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
    action.send(Action::Stop).unwrap();
    eprintln!("waiting for thread to stop...");
    thread.join().unwrap().unwrap()
}
