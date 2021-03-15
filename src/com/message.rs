use crate::dhtm::msg::{MessageCommand, MessageKey, MessageType};
use crate::dhtm::msg::{
    CMD_OFFSET, DEF_PL_SIZE, ID_OFFSET, KEY_OFFSET, PAYLOAD_OFFSET, TYPE_OFFSET,
};
use crate::num_traits::{FromPrimitive, ToPrimitive};

pub struct Message {
    pub data: Vec<u8>,
}

impl Message {
    pub fn create_header(
        &mut self,
        msg_type: MessageType,
        msg_cmd: MessageCommand,
        msg_key: MessageKey,
    ) {
        if self.data.len() < PAYLOAD_OFFSET {
            self.data = vec![0; PAYLOAD_OFFSET];
        }
        self.set_type(msg_type);
        self.set_cmd(msg_cmd);
        self.set_key(msg_key);
    }

    pub fn parse_to(&self, sdr: &mut Vec<bool>) {
        for i in 0..sdr.len() {
            let byte = i >> 3 + PAYLOAD_OFFSET;
            let bit = i % 8;
            sdr[i] = self.data[byte] & (1 << bit) != 0;
        }
    }

    pub fn set_payload_bit(&mut self, idx: &usize) {
        let byte = (idx >> 3) + PAYLOAD_OFFSET;
        let bit = idx % 8;
        self.data[byte] |= 1 << bit;
    }

    pub fn unset_payload_bit(&mut self, idx: &usize) {
        let byte = (idx >> 3) + PAYLOAD_OFFSET;
        let bit = idx % 8;
        self.data[byte] &= !(1 << bit);
    }

    pub fn print(&self) -> std::string::String {
        return format!(
            ">> MSG - ID: {}, TYPE: {}, CMD: {}, KEY: {}",
            self.get_prop(&ID_OFFSET),
            self.get_prop(&TYPE_OFFSET),
            self.get_prop(&CMD_OFFSET),
            self.get_prop(&KEY_OFFSET)
        );
    }

    pub fn get_prop(&self, offset: &usize) -> u16 {
        return u16::from_be_bytes([self.data[*offset], self.data[*offset + 1]]);
    }

    pub fn get_type(&self) -> Option<MessageType> {
        return MessageType::from_u16(self.get_prop(&TYPE_OFFSET));
    }

    pub fn get_cmd(&self) -> Option<MessageCommand> {
        return MessageCommand::from_u16(self.get_prop(&CMD_OFFSET));
    }

    pub fn get_key(&self) -> Option<MessageKey> {
        return MessageKey::from_u16(self.get_prop(&KEY_OFFSET));
    }

    pub fn set_prop(&mut self, offset: &usize, prop: &u16) {
        let raw_prop = prop.to_be_bytes();
        self.data[*offset] = raw_prop[0];
        self.data[*offset + 1] = raw_prop[1];
    }

    pub fn set_type(&mut self, msg_type: MessageType) {
        if let Some(v) = msg_type.to_u16() {
            self.set_prop(&TYPE_OFFSET, &v)
        }
    }

    pub fn set_cmd(&mut self, msg_cmd: MessageCommand) {
        if let Some(v) = msg_cmd.to_u16() {
            self.set_prop(&CMD_OFFSET, &v)
        }
    }

    pub fn set_key(&mut self, msg_key: MessageKey) {
        if let Some(v) = msg_key.to_u16() {
            self.set_prop(&KEY_OFFSET, &v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_payload_bit() {
        let byte = 3;
        let offset = PAYLOAD_OFFSET + byte;
        let input: u8 = 0b0000_1111;
        let idx = (byte << 3) + 5;
        let expected: u8 = 0b0010_1111;
        let mut m = Message {
            data: vec![0; DEF_PL_SIZE + PAYLOAD_OFFSET],
        };
        m.data[offset] = input;
        m.set_payload_bit(&(idx as usize)); // Set 0 to 1
        assert_eq!(m.data[offset], expected);
        m.set_payload_bit(&((idx - 3) as usize)); // 1 remains set
        assert_eq!(m.data[offset], expected);
    }

    #[test]
    fn test_unset_payload_bit() {
        let byte = 3;
        let offset = PAYLOAD_OFFSET + byte;
        let input: u8 = 0b0000_1111;
        let idx = (byte << 3) + 0;
        let expected: u8 = 0b0000_1110;
        let mut m = Message {
            data: vec![0; DEF_PL_SIZE + PAYLOAD_OFFSET],
        };
        m.data[offset] = input;
        m.unset_payload_bit(&(idx as usize)); // Set 1 to 0
        assert_eq!(m.data[offset], expected);
        m.unset_payload_bit(&((idx + 5) as usize)); // 0 remains set
        assert_eq!(m.data[offset], expected);
    }
}
