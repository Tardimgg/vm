use crate::operand::Operand;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Copy, Clone)]
pub struct Command {
    pub command_type: CommandType,
    pub operand: Operand
}

#[derive(Debug, EnumString, IntoStaticStr, EnumIter, Copy, Clone)]
#[strum(ascii_case_insensitive)]
pub enum CommandType {
    NOP,
    JMP,
    ADD,
    SUB,
    LAC,
    DAC,
    CMP,
    MOV,
    LRG,
    JMPG,
    JMPNG,
    MULT
}

impl CommandType {

    pub const fn get_code(&self) -> u16 {
        match self {
            CommandType::NOP => 0,
            CommandType::JMP => 1,
            CommandType::ADD => 2,
            CommandType::LAC => 3,
            CommandType::DAC => 4,
            CommandType::SUB => 5,
            CommandType::CMP => 6,
            CommandType::MOV => 7,
            CommandType::LRG => 8,
            CommandType::JMPG => 9,
            CommandType::JMPNG => 10,
            CommandType::MULT => 11
        }
    }

}