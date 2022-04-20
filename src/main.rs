use std::time::Instant;
use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Write;

use crossbeam_channel::unbounded;
use std::{thread};

use once_cell::sync::OnceCell;

static CLIENT_KE_S: OnceCell<crossbeam_channel::Sender<u128>> = OnceCell::new();

fn main() {
    // Create an unbounded channel
    let (client_ke_s, client_ke_r) = unbounded();

    CLIENT_KE_S.set(client_ke_s).expect("unable to fill once cell.");

    // make a thread for writing client_ke meas
    thread::spawn(move || {
        let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open("results/client_nts_ke")
        .expect("Unable to create file");

        loop {
            // get and write measurements
            let value = client_ke_r.recv().unwrap();
            writeln!(f, "{}", value).expect("Unable to write file");
        }
    });


    for _ in 1..500 {
        let start = Instant::now();

        // 2x for start and end
        let _bench = Instant::now();
        let _bench = Instant::now();

        let end = Instant::now();

        let time_meas_nanos = (end - start).as_nanos();

        CLIENT_KE_S.get().clone().unwrap().send(time_meas_nanos).expect("unable to write to channel.");
    }

    // give the writer a chance to write before exiting (mimics real code)
    thread::sleep(Duration::from_millis(1000));
}

