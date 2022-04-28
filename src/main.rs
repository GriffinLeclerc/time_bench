use std::io::Read;
use std::time::Instant;
use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Write;

use crossbeam_channel::unbounded;
use std::{thread};

use once_cell::sync::OnceCell;
use std::env;
use std::fs::File;

static CLIENT_KE_S: OnceCell<crossbeam_channel::Sender<u128>> = OnceCell::new();

fn main() {
    let path = env::current_dir().unwrap();
    println!("{}", path.display());

    let settings = config::Config::builder()
            .add_source(config::File::with_name("experiment.yaml")).build().expect("Unable to build ntp server config.");

    // num_runs: 1
    // exchanges_per_cookie: 1
    
    // min_clients: 1
    // max_clients: 3000
    // step_size: 10
    

    let step_size = settings.get_string("step_size").unwrap().parse::<i32>().unwrap();

    let mut file = File::open("next_run").unwrap();
    let mut tmp = String::new();
    file.read_to_string(&mut tmp).expect("Unable to read next number of clients");
    let mut next_run = tmp.parse::<i32>().unwrap();

    println!("{}", next_run);

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


    for i in 0..next_run {
        println!("{}", i);

        let start = Instant::now();

        // 2x for start and end
        let _bench = Instant::now();
        let _bench = Instant::now();

        let end = Instant::now();

        let time_meas_nanos = (end - start).as_nanos();

        CLIENT_KE_S.get().clone().unwrap().send(time_meas_nanos).expect("unable to write to channel.");
    }
    
    let mut file = File::create("next_run").unwrap();
    next_run += step_size;
    file.write_all(next_run.to_string().as_bytes()).expect("Unable to write next run");

    println!("{}", next_run);
}

