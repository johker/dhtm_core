extern crate htm;
extern crate rand;
extern crate time;

use htm::{SpatialPooler, UniversalNext, UniversalRng};
use std::convert::TryFrom;
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

pub fn utf8_to_string(bytes: &[u8]) -> String {
    let vector: Vec<u8> = Vec::from(bytes);
    String::from_utf8(vector).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let bytes: [u8; 7] = [0x55, 0x54, 0x46, 0x38, 0, 0, 0];
        let len: usize = 4;
        let actual = utf8_to_string(&bytes[0..len]);
        assert_eq!("UTF8", actual);
    }
}

fn main() {
    println!("Initializing Message Broker");
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
            // let string = subscriber.recv_string(0).unwrap().unwrap();
            let s = subscriber.recv_bytes(0).unwrap();
            // println!("RECV ZMQ: {}", string);
            tx.send(s).unwrap();
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
        println!("Initializing Spatial Pooler ...");
        println!("Skipped");
        //let start = PreciseTime::now();
        //sp.init();
        //println!(": {:?}", start.to(PreciseTime::now()));
        //println!("Done.");
    }

    let mut rnd = UniversalRng::from_seed([42, 0, 0, 0]);
    let mut input = vec![false; sp.num_inputs];

    let mut msg_id: u16;
    let mut msg_type: u16;
    let mut msg_cmd: u16;
    let mut msg_key: u16;

    for received in rx {
        println!("Received new message: {:?}", received);
        //let split = received.split(",");
        //let vec = received.collect::<Vec<&str>>();
        println!("Vector size: {:?}", received.len());
        if received.len() > 1 {
            msg_id = u16::from_be_bytes([received[ID_OFFSET], received[ID_OFFSET + 1]]);
            msg_type = u16::from_be_bytes([received[TYPE_OFFSET], received[TYPE_OFFSET + 1]]);
            msg_cmd = u16::from_be_bytes([received[CMD_OFFSET], received[CMD_OFFSET + 1]]);
            msg_key = u16::from_be_bytes([received[KEY_OFFSET], received[KEY_OFFSET + 1]]);

            println!(
                "ID: {}, TYPE: {}, CMD: {}, KEY: {}",
                msg_id, msg_type, msg_cmd, msg_key
            );
            // Dummy input

            // TODO Get from message
            for val in &mut input {
                *val = rnd.next_uv_int(2) == 1;
            }
            //let mut data: [u8; 4096 >> 3] = [0; 4096 >> 3];
            let mut data: [u8; 32] = [0; 32];

            // Compute next acitvation
            println!("Computing update ...");
            //sp.compute(&input, true);
            //println!("Done.");
            println!("Skipped");

            // Bits to flip to 1:
            let test_cols = [0, 1, 8, 9];

            for col_idx in test_cols.iter() {
                //sp.winner_columns.iter() {
                let byte_idx = col_idx >> 3;
                let bit_idx = col_idx % 8;
                data[byte_idx] = data[byte_idx] | 1 << bit_idx;
                println!("data[{}] = {}", byte_idx, data[byte_idx]);
            }

            // Convert to output message format
            // Send update (if requested)

            // let s = String::from_utf8(data.to_vec()).unwrap();
            println!("SENT ZMQ: {:?}", data);
            if msg_id == 2 {
                publisher.send(&data.to_vec(), 0).unwrap();
            }
        } // End of vector size check
    }
}
