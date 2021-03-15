extern crate htm;
extern crate rand;
extern crate time;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use num_traits::{FromPrimitive, ToPrimitive};

use htm::{SpatialPooler, UniversalNext, UniversalRng};
use std::convert::TryFrom;
use time::PreciseTime;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Include auto-generated file
mod com;
#[path = "../dhtm_msg/rs/msg.rs"]
mod dhtm;

use self::com::message::Message;
use self::dhtm::msg::{MessageCommand, MessageKey, MessageType};
use self::dhtm::msg::{
    CMD_OFFSET, DEF_PL_SIZE, ID_OFFSET, KEY_OFFSET, PAYLOAD_OFFSET, TYPE_OFFSET,
};

fn main() {
    println!("Initializing Message Broker");
    let (tx, rx) = mpsc::channel();

    let context = zmq::Context::new();

    let mut m = Message {
        data: vec![0; DEF_PL_SIZE + PAYLOAD_OFFSET],
    };

    m.create_header(
        MessageType::DATA,
        MessageCommand::PRINT,
        MessageKey::S_SPOOL,
    );

    // Initialize publisher
    let publisher = context.socket(zmq::PUB).unwrap();
    assert!(publisher.connect("tcp://localhost:6000").is_ok());

    // Initialize subsciber
    thread::spawn(move || {
        let subscriber = context.socket(zmq::SUB).unwrap();
        assert!(subscriber.connect("tcp://localhost:5555").is_ok());
        let filter = MessageType::DATA as u8;
        subscriber
            .set_subscribe(&filter.to_string().as_bytes())
            .expect("Failed to subscribe");
        println!("Subscribed to {:?}", filter.to_string().as_bytes());
        // subscriber.set_subscribe(b"").expect("Failed to subscribe");

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
        let start = PreciseTime::now();
        sp.init();
        println!(": {:?}", start.to(PreciseTime::now()));
        println!("Done.");
    }

    let mut input = vec![false; sp.num_inputs];

    for received in rx {
        println!("Received new message: {:?}", received);
        println!("Vector size: {:?}", received.len());
        if received.len() > 1 {
            m.data = received;
            println!("RECV {}", m.print());
            if let Some(MessageCommand::INPUT) = m.get_cmd() {
                println!("Input: {:?}", input);
                m.parse_to(&mut input);
                println!("Input: {:?}", input);

                // Compute next acitvation
                println!("Computing update ...");
                sp.compute(&input, true);
                println!("Done.");
                //println!("Skipped");

                m.create_header(
                    MessageType::DATA,
                    MessageCommand::PRINT,
                    MessageKey::S_SPOOL,
                );

                // Set payload
                // Bits to flip to 1:
                for col_idx in sp.winner_columns.iter() {
                    let byte_idx = col_idx >> 3 + dhtm::msg::PAYLOAD_OFFSET;
                    let bit_idx = col_idx % 8;
                    m.data[byte_idx] = m.data[byte_idx] | 1 << bit_idx;
                    // println!("data[{}] = {}", byte_idx, data[byte_idx]);
                }

                // Convert to output message format

                // let s = String::from_utf8(data.to_vec()).unwrap();
                publisher.send(&m.data, 0).unwrap();
            }
        } // End of vector size check
    }
}
