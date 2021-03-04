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
#[path = "../dhtm_msg/rs/msg.rs"]
mod dhtm;

struct Message {
    data: Vec<u8>,
}

impl Message {
    fn create_header(
        &mut self,
        msg_type: dhtm::msg::MessageType,
        msg_cmd: dhtm::msg::MessageCommand,
        msg_key: dhtm::msg::MessageKey,
    ) {
        self.set_type(msg_type);
        self.set_cmd(msg_cmd);
        self.set_key(msg_key);
    }

    fn parse(&mut self, raw_data: &[u8]) {
        self.data = raw_data.to_vec();
    }

    fn print(&self) -> std::string::String {
        return format!(
            ">> MSG - ID: {}, TYPE: {}, CMD: {}, KEY: {}",
            self.get_prop(&dhtm::msg::ID_OFFSET),
            self.get_prop(&dhtm::msg::TYPE_OFFSET),
            self.get_prop(&dhtm::msg::CMD_OFFSET),
            self.get_prop(&dhtm::msg::KEY_OFFSET)
        );
    }

    fn get_prop(&self, offset: &usize) -> u16 {
        return u16::from_be_bytes([self.data[*offset], self.data[*offset + 1]]);
    }

    fn get_type(&self) -> Option<dhtm::msg::MessageType> {
        return dhtm::msg::MessageType::from_u16(self.get_prop(&dhtm::msg::TYPE_OFFSET));
    }

    fn get_cmd(&self) -> Option<dhtm::msg::MessageCommand> {
        return dhtm::msg::MessageCommand::from_u16(self.get_prop(&dhtm::msg::CMD_OFFSET));
    }

    fn get_key(&self) -> Option<dhtm::msg::MessageKey> {
        return dhtm::msg::MessageKey::from_u16(self.get_prop(&dhtm::msg::KEY_OFFSET));
    }

    fn set_prop(&mut self, offset: &usize, prop: &u16) {
        let raw_prop = prop.to_be_bytes();
        self.data[*offset] = raw_prop[raw_prop.len() - 2];
        self.data[*offset + 1] = raw_prop[raw_prop.len() - 1];
    }

    fn set_type(&mut self, msg_type: dhtm::msg::MessageType) {
        if let Some(v) = msg_type.to_u16() {
            self.set_prop(&dhtm::msg::TYPE_OFFSET, &v)
        }
    }

    fn set_cmd(&mut self, msg_cmd: dhtm::msg::MessageCommand) {
        if let Some(v) = msg_cmd.to_u16() {
            self.set_prop(&dhtm::msg::CMD_OFFSET, &v)
        }
    }

    fn set_key(&mut self, msg_key: dhtm::msg::MessageKey) {
        if let Some(v) = msg_key.to_u16() {
            self.set_prop(&dhtm::msg::KEY_OFFSET, &v)
        }
    }
}

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
        let filter = dhtm::msg::MessageType::DATA as u8;
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
        println!("Skipped");
        //let start = PreciseTime::now();
        //sp.init();
        //println!(": {:?}", start.to(PreciseTime::now()));
        //println!("Done.");
    }

    let mut rnd = UniversalRng::from_seed([42, 0, 0, 0]);
    let mut input = vec![false; sp.num_inputs];

    let mut recv_msg = Message {
        data: Vec::<u8>::new(),
    };

    for received in rx {
        println!("Received new message: {:?}", received);
        //let split = received.split(",");
        //let vec = received.collect::<Vec<&str>>();
        println!("Vector size: {:?}", received.len());
        if received.len() > 1 {
            let m = Message { data: received };
            println!("RECV {}", m.print());

            // Dummy input

            // TODO Get from message
            for val in &mut input {
                *val = rnd.next_uv_int(2) == 1;
            }
            //let mut data: [u8; 4096 >> 3] = [0; 4096 >> 3];
            let mut data: [u8; 36] = [0; 36];

            // Compute next acitvation
            println!("Computing update ...");
            //sp.compute(&input, true);
            //println!("Done.");
            println!("Skipped");

            let mut m = Message {
                data: Vec::<u8>::new(),
            };
            m.create_header(
                dhtm::msg::MessageType::DATA,
                dhtm::msg::MessageCommand::PRINT,
                dhtm::msg::MessageKey::S_SPOOL,
            );

            // Set payload
            // Bits to flip to 1:
            let test_cols = [0, 1, 8, 9];
            for col_idx in test_cols.iter() {
                //sp.winner_columns.iter() {
                let byte_idx = col_idx >> 3 + dhtm::msg::PAYLOAD_OFFSET;
                let bit_idx = col_idx % 8;
                m.data[byte_idx] = data[byte_idx] | 1 << bit_idx;
                // println!("data[{}] = {}", byte_idx, data[byte_idx]);
            }

            // Convert to output message format
            // Send update (if requested)

            // let s = String::from_utf8(data.to_vec()).unwrap();
            if let Some(dhtm::msg::MessageCommand::INPUT) = m.get_cmd() {
                //println!("SENT ZMQ: {:?}", data[0..31]);
                publisher.send(&m.data, 0).unwrap();
            }
        } // End of vector size check
    }
}
