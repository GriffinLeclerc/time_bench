extern crate time;

use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open("res")
        .expect("Unable to create file");

    let start = Instant::now();

    // 2x for start and end
    let _bench = Instant::now();
    let _bench = Instant::now();

    let end = Instant::now();

    let time_meas_nanos = end - start;

    writeln!(f, "{}", time_meas_nanos.as_nanos()).expect("Unable to write file");
    // println!("{}", time_meas_nanos.as_nanos());

    // f.write_all(&time_meas_nanos.to_be_bytes()).expect
    
}

