use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Default)]
pub struct DataMemory {
    data: HashMap<u16, u8>
}

impl DataMemory {

    pub fn dump(&self) -> [u8; 1 << 15] {
        let mut res = [0u8; 1 << 15];
        for i in 0u16..(1 << 15) {
            res[i as usize] = *self.data.get(&i).unwrap_or(&0u8);
        }

        res
    }

    pub fn restore(dump: &[u8]) -> Self {
        let mut data = HashMap::new();
        for i in 0u16..min(u16::MAX as usize, dump.len()) as u16 {
            data.insert(i, dump[i as usize]);
        }

        Self { data }
    }
}

impl DataMemory {

    pub fn get(&self, address: u16) -> u8 {
        *self.data.get(&address).unwrap_or(&0)
    }

    pub fn put(&mut self, address: u16, value: u8) {
        self.data.insert(address, value);
    }

    pub fn put_bytes(&mut self, address: u16, value: &[u8]) {
        let max_address = address + value.len() as u16;
        for index in address..max_address {
            self.data.insert(index, value[(index - address) as usize]);
        }
    }
}