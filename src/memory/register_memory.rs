use crate::register::{Register, RegisterName};
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct RegisterMemory {
    data: HashMap<RegisterName, Register>
}

impl RegisterMemory {

    pub fn new() -> RegisterMemory {
        let mut data = HashMap::new();

        for register_name in RegisterName::iter() {
            data.insert(register_name, register_name.new_register());
        }
        RegisterMemory {
            data,
        }
    }

    pub fn get(&self, id: RegisterName) -> Option<&Register> {
        self.data.get(&id)
    }

    pub fn get_mut(&mut self, id: RegisterName) -> Option<&mut Register> {
        self.data.get_mut(&id)
    }

    pub fn get_all(&self) -> &HashMap<RegisterName, Register> {
        &self.data
    }
}