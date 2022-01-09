#![allow(unused)]
extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use mac_disk_monitor::event::Event;
use mac_disk_monitor::std::*;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

fn main() {
    let app = App::new("mac-disk-monitor");
    let matches = app.get_matches();

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
                eprintln!("warning: {}", e);
            }
        }
    }
}
