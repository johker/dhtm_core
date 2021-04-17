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
#[path = "../dhtm_msg/rs/msg.rs"]
mod dhtm;
mod node;

use self::dhtm::msg::{MessageCommand, MessageKey, MessageType};
use self::dhtm::msg::{DEF_PL_SIZE, PAYLOAD_OFFSET};
use self::node::com::Message;
use self::node::ctrl::{Event, State};
use self::node::repl::read_lines;

fn main() {
    println!("Initializing Message Broker");

    let (tx, rx) = mpsc::channel();
    let context = zmq::Context::new();
    let mut state = State::Waiting { waiting_time: 0 };
    let mut m = Message {
        data: vec![0; DEF_PL_SIZE + PAYLOAD_OFFSET],
    };

    // Initialize publisher
    let publisher = context.socket(zmq::PUB).unwrap();
    assert!(publisher.connect("tcp://localhost:6000").is_ok());

    // Initialize subsciber
    thread::spawn(move || {
        let subscriber = context.socket(zmq::SUB).unwrap();
        assert!(subscriber.connect("tcp://localhost:5555").is_ok());
        subscriber
            .set_subscribe(
                &format!(
                    "T{:03}.{:03}",
                    MessageType::CONFIGURATION as u16,
                    MessageCommand::REPLICATE as u16
                )
                .as_bytes(),
            )
            .expect("Failed to subscribe");

        subscriber
            .set_subscribe(
                &format!(
                    "T{:03}.{:03}",
                    MessageType::DATA as u16,
                    MessageCommand::WRITE as u16
                )
                .as_bytes(),
            )
            .expect("Failed to subscribe");

        loop {
            let s = subscriber.recv_bytes(0).unwrap();
            tx.send(s).unwrap();
        }
    });

    // Sending test message
    println!("Sending test message");
    m.set_type(MessageType::CONFIGURATION);
    m.set_cmd(MessageCommand::REPLICATE);
    publisher.send(&m.get_topic(), zmq::SNDMORE).unwrap();
    publisher.send(&m.data, 0).unwrap();
    //    Initialize SpatialPooler
    //    let mut sp = SpatialPooler::new(vec![32, 32], vec![64, 64]);
    //    sp.potential_radius = sp.num_inputs as i32;
    //    sp.global_inhibition = true;
    //    sp.num_active_columns_per_inh_area = 0.02 * sp.num_columns as f64;
    //    sp.syn_perm_options.active_inc = 0.01;
    //    sp.syn_perm_options.trim_threshold = 0.005;
    //    sp.compability_mode = true;

    //{
    //    println!("Initializing Spatial Pooler ...");
    //    sp.init();
    //    println!("Done.");
    //}

    //let mut input = vec![false; sp.num_inputs];

    for received in rx {
        if received.len() > 1 {
            // println!("RAW: {:?}", received);
            if received[0] == 84 {
                // Starts with T => Topic
                continue;
            }

            m.data = received;
            println!("RECV MSG (TOPIC: {})", m.get_topic());

            // Actions are always triggered by messages
            match m.get_cmd() {
                Some(MessageCommand::INPUT) => {
                    //     m.parse_to(&mut input);

                    //     // Compute next acitvation
                    //     sp.compute(&input, true);

                    //     m.create_header(
                    //         MessageType::DATA,
                    //         MessageCommand::PRINT,
                    //         MessageKey::S_SPOOL,
                    //     );

                    //     // Set payload
                    //     // println!("Winner Columns {:?}", sp.winner_columns);
                    //     for col_idx in sp.winner_columns.iter() {
                    //         m.set_payload_bit(col_idx);
                    //     }

                    //     println!("SENT MSG (TOPIC: {})", m.get_topic());
                    //     // println!("SENT {}", m.print());
                    //     publisher.send(&m.get_topic(), zmq::SNDMORE).unwrap();
                    //     publisher.send(&m.data, 0).unwrap();
                }
                Some(MessageCommand::REPLICATE) => {
                    if let Ok(lines) = read_lines("/home/workspace/dhtm_core/src/main.rs") {
                        // Consumes the iterator, returns an (Optional) String
                        for line in lines {
                            if let Ok(ip) = line {
                                println!("{}", ip);
                            }
                        }
                    }
                }
                None => println!("Invalid Command"),
                _ => println!("Unhandled command"),
            } // End of match
        } // End of vector size check
    }
}
