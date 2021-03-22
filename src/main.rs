extern crate htm;
extern crate rand;
extern crate time;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use crate::num_traits::{FromPrimitive, ToPrimitive};
use htm::SpatialPooler;

use std::sync::mpsc;
use std::thread;

// Include auto-generated file
mod com;
#[path = "../dhtm_msg/rs/msg.rs"]
mod dhtm;

use self::com::message::Message;
use self::dhtm::msg::{MessageCommand, MessageKey, MessageType};
use self::dhtm::msg::{DEF_PL_SIZE, PAYLOAD_OFFSET};

fn main() {
    println!("Initializing Message Broker");

    let (tx, rx) = mpsc::channel();
    let context = zmq::Context::new();
    let mut m = Message {
        data: vec![0; DEF_PL_SIZE + PAYLOAD_OFFSET],
    };

    m.create_header(
        MessageType::DATA,
        MessageCommand::INPUT,
        MessageKey::S_SPOOL,
    );

    // Initialize publisher
    let publisher = context.socket(zmq::PUB).unwrap();
    assert!(publisher.connect("tcp://localhost:6000").is_ok());

    // Initialize subsciber
    let sub_topic = m.get_topic();
    thread::spawn(move || {
        let subscriber = context.socket(zmq::SUB).unwrap();
        assert!(subscriber.connect("tcp://localhost:5555").is_ok());
        subscriber
            .set_subscribe(&sub_topic.as_bytes())
            .expect("Failed to subscribe");
        println!("Subscribed to {:?}", sub_topic.as_bytes());

        loop {
            let s = subscriber.recv_bytes(0).unwrap();
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
        sp.init();
        println!("Done.");
    }

    let mut input = vec![false; sp.num_inputs];

    for received in rx {
        if received.len() > 1 {
            println!("RAW: {:?}", received);
            if received[0] == 84 {
                // Topic
                continue;
            }
            m.data = received;
            println!("RECV {}", m.print());
            if let Some(MessageCommand::INPUT) = m.get_cmd() {
                m.parse_to(&mut input);

                // Compute next acitvation
                println!("Computing update ...");
                sp.compute(&input, true);
                println!("Done.");

                m.create_header(
                    MessageType::DATA,
                    MessageCommand::PRINT,
                    MessageKey::S_SPOOL,
                );

                // Set payload
                // println!("Winner Columns {:?}", sp.winner_columns);
                for col_idx in sp.winner_columns.iter() {
                    m.set_payload_bit(col_idx);
                }

                let pub_topic = MessageCommand::PRINT;
                println!("TOPIC {}", m.get_topic());
                println!("SENT {}", m.print());
                publisher.send(&m.get_topic(), zmq::SNDMORE).unwrap();
                publisher.send(&m.data, 0).unwrap();
            }
        } // End of vector size check
    }
}
