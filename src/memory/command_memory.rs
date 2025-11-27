use crate::command::Command;
use std::collections::HashMap;
use std::rc::Rc;
use crate::byte_formatter::byte_formatter::ByteFormatter;
use crate::decoder::bin_decoder::parse_command;

pub struct CommandMemory {
    data: HashMap<u16, u8>,
    bytes_formatter: Rc<dyn ByteFormatter>
}

impl CommandMemory {

    pub fn load(commands: Vec<u8>, bytes_formatter: Rc<dyn ByteFormatter>) -> CommandMemory {

        let command_map = commands.into_iter().enumerate()
            .map(|(address, cmd)| (address as u16, cmd))
            .collect();

        CommandMemory {
            data: command_map,
            bytes_formatter
        }
    }

    pub fn get(&self, address: u16) -> Result<Option<Command>, String> {
        let first_byte = self.data.get(&address).unwrap_or(&0);
        let second_byte = self.data.get(&(address + 1)).unwrap_or(&0);

        let cmd = parse_command(&[*first_byte, *second_byte], self.bytes_formatter.clone())?;

        Ok(Some(cmd))
    }

    pub fn get_all(&self) -> Vec<(u16, Command)> {
        let indexes = self.data.keys();

        indexes.into_iter()
            .filter(|v| **v % 2 == 0)
            .map(|v| (*v, self.get(*v)))
            .filter(|(_, v)| v.is_ok() && v.as_ref().unwrap().is_some())
            .map(|(i, v)| (i, v.unwrap().unwrap()))
            .collect()
    }
}