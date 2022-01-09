use mac_disk_monitor::stream_events;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let (action, receiver) = channel();
    let (thread, receiver) = stream_events(receiver);

    loop {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                Some(event) => {
                    println!("{}", event.to_json());
                }
                None => {}
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
