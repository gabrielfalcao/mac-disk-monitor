use mac_disk_monitor::{stream_events, Action};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let (action, receiver) = channel();
    let (thread, receiver) = stream_events(receiver);

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
                action.send(Action::Stop).unwrap();
                eprintln!("Error: {}", e);
            }
        }
    }
    thread.join().unwrap().unwrap();
    println!("done!")
}
