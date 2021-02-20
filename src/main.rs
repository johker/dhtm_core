extern crate htm;
extern crate rand;
extern crate time;

use htm::{SpatialPooler, UniversalNext, UniversalRng};
use time::PreciseTime;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// TODO: Auto-Generate
const ID_OFFSET: usize = 0;
const TYPE_OFFSET: usize = 2;
const CMD_OFFSET: usize = 4;
const KEY_OFFSET: usize = 6;
const PAYLOAD_OFFSET: usize = 8;

fn atoi(s: &str) -> i64 {
    s.parse().unwrap()
}

fn main() {
    println!("Initializing message broker");
    let (tx, rx) = mpsc::channel();

    let context = zmq::Context::new();

    // Initialize publisher
    let publisher = context.socket(zmq::PUB).unwrap();
    assert!(publisher.connect("tcp://localhost:6000").is_ok());

    // Initialize subsciber
    thread::spawn(move || {
        let subscriber = context.socket(zmq::SUB).unwrap();
        assert!(subscriber.connect("tcp://localhost:5555").is_ok());
        //let filter = "10001";
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

    {
        print!("Initializing Spatial Pooler ...");
        let start = PreciseTime::now();
        sp.init();
        println!(": {:?}", start.to(PreciseTime::now()));
    }

    let mut rnd = UniversalRng::from_seed([42, 0, 0, 0]);
    let mut input = vec![false; sp.num_inputs];

    let mut msg_id: u16;
    let mut msg_type: u16;
    let mut msg_cmd: u16;
    let mut msg_key: u16;

    for received in rx {
        println!("Received new message: {:?}", received);
        let split = received.split(",");
        let vec = split.collect::<Vec<&str>>();

        let id_bytes: [u8; 2] = [
            vec[ID_OFFSET].parse().unwrap(),
            vec[ID_OFFSET + 1].parse().unwrap(),
        ];
        let type_bytes: [u8; 2] = [
            vec[TYPE_OFFSET].parse().unwrap(),
            vec[TYPE_OFFSET + 1].parse().unwrap(),
        ];
        let cmd_bytes: [u8; 2] = [
            vec[CMD_OFFSET].parse().unwrap(),
            vec[CMD_OFFSET + 1].parse().unwrap(),
        ];
        let key_bytes: [u8; 2] = [
            vec[KEY_OFFSET].parse().unwrap(),
            vec[KEY_OFFSET + 1].parse().unwrap(),
        ];
        msg_id = u16::from_be_bytes(id_bytes);
        msg_type = u16::from_be_bytes(type_bytes);
        msg_cmd = u16::from_be_bytes(cmd_bytes);
        msg_key = u16::from_be_bytes(key_bytes);
        println!(
            "ID: {}, TYPE: {}, CMD: {}, KEY: {}",
            msg_id, msg_type, msg_cmd, msg_key
        );

        for (i, s) in vec.iter().enumerate() {
            let byte: u8 = s.parse().unwrap();
            // Bitwise operation to extract bool
        }

        // Dummy input

        // TODO Get from message
        for val in &mut input {
            *val = rnd.next_uv_int(2) == 1;
        }
        let mut data: [u8; 4096 >> 3] = [0; 4096 >> 3];

        // Compute next acitvation
        println!("Computing update ...");
        sp.compute(&input, true);
        println!("Done.");

        for col_idx in sp.winner_columns.iter() {
            let byte_idx = col_idx >> 3;
            let bit_idx = col_idx % 8;
            data[byte_idx] = data[byte_idx] | 1 << bit_idx;
        }
        // Convert to output message format
        // Send update (if requested)
        let update = "";
        publisher.send(&update, 0).unwrap();
    }
}
