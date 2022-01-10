extern crate clap;
use clap::{App, Arg};

use ctrlc;
use mac_disk_monitor::std::*;
use mac_disk_monitor::version;
use std::sync::mpsc::channel;
use std::time::Duration;

/// Command-line entrypoint to monitor the disk events and print them out to stdout as json.
#[cfg(not(tarpaulin_include))]
fn main() {
    let version = version();
    let app = App::new("mac-disk-monitor")
        .version(version.as_str())
        .about("command-line tool to monitor disk activity on MacOS and output as json")
        .arg(
            Arg::with_name("format")
                .long("format")
                .help("the output format: (json|yaml)")
                .short("f")
                .default_value("json")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .help("how many milliseconds to wait for each event")
                .short("t")
                .default_value("3145")
                .required(true)
                .takes_value(true),
        );
    let matches = app.get_matches();

    let interval = matches
        .value_of("interval")
        .unwrap_or("")
        .parse::<u64>()
        .unwrap_or(3145);

    let format = matches.value_of("format").unwrap_or("json");
    match format {
        "json" | "yaml" => {}
        other => {
            eprintln!(
                "Invalid format {:?} should be either 'json' or 'yaml'",
                other
            );
            std::process::exit(1);
        }
    }

    let (action, receiver) = channel();
    let (thread, receiver) = stream_events(receiver);
    ctrlc::set_handler(move || {
        action
            .send(Action::Stop)
            .expect("Could not send 'stop' action to thread.");
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        match receiver.recv_timeout(Duration::from_millis(interval)) {
            Ok(event) => match event {
                Some(event) => {
                    println!(
                        "{}",
                        match format {
                            "yaml" => event.to_yaml(),
                            _ => event.to_json(),
                        }
                    );
                }
                None => {
                    break;
                }
            },
            Err(e) => match e.to_string().as_str() {
                "timed out waiting on channel" => {}
                "channel is empty and sending half is closed" => {
                    break;
                }
                error => {
                    eprintln!("Error: {}", error);
                    break;
                }
            },
        }
    }
    eprintln!("waiting for thread to stop...");
    thread.join().unwrap().unwrap();
    eprintln!("done");
}
