extern crate htm;

use htm::{SpatialPooler, UniversalNext, UniversalRng};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const ID_OFFSET: u16 = 0;

fn atoi(s: &str) -> i64 {
    s.parse().unwrap()
}

fn main() {
    println!("Hello, world!");
    let (tx, rx) = mpsc::channel();

    let context = zmq::Context::new();

    thread::spawn(move || {
        let subscriber = context.socket(zmq::SUB).unwrap();
        assert!(subscriber.connect("tcp://localhost:5555").is_ok());
        let filter = "10001";
        //assert!(subscriber.set_subscribe(filter.as_bytes()).is_ok());
        subscriber.set_subscribe(b"").expect("Failed to subscribe");

        loop {
            let string = subscriber.recv_string(0).unwrap().unwrap();
            //let s = subscriber.recv_bytes(0).unwrap();
            //println!("{:?}", s);
            println!("{}", string);
            tx.send(string).unwrap();
        }
    });

    // Initialize SpatialPooler
    let mut sp = SpatialPooler::new(vec![32, 32], vec![64, 64]);
    sp.potential_radius = sp.num_inputs as i32;
    sp.global_inhibition = true;
    sp.num_active_columns_per_inh_area = 0.02 * sp.num_columns as f64;
    sp.syn_perm_options.active_inc = 0.01;
    sp.syn_perm_options.trim_threshold = 0.005;
    sp.compability_mode = true;

    let mut msg_id: u16;
    let mut msg_type: u16;
    let mut msg_cmd: u16;
    let mut msg_key: u16;

    for received in rx {
        println!("Got: {:?}", received);
        let split = received.split(",");
        let vec = split.collect::<Vec<&str>>();
        // TODO: parse bytes based on offsets
        let id_bytes: [u8; 2] = [vec[0].parse().unwrap(), vec[1].parse().unwrap()];
        let mid = u16::from_be_bytes(id_bytes);
        println!("ID: {}", mid);
        //println!("ID = {}", msg_id);
        // Compute next acitvation

        // Send back activated columns
    }
}
