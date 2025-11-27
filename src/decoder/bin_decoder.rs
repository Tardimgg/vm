use crate::byte_formatter::byte_formatter::ByteFormatter;
use crate::command::{Command, CommandType};
use crate::operand::{Operand, OperandTag};
use crate::register::{Register, RegisterName};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::rc::Rc;
use enum_tags::TaggedEnum;
use strum::IntoEnumIterator;

pub fn parse_bin(bytes: &[u8], byte_formatter: Rc<dyn ByteFormatter>) -> Result<Vec<Command>, String> {
    if bytes.len() % 2 != 0 {
        return Err("invalid data".to_string());
    }

    let mut res = Vec::new();

    for command in bytes.chunks(2) {
        res.push(parse_command(command, byte_formatter.clone())?);
    }

    Ok(res)
}

pub fn parse_command(bin: &[u8], byte_formatter: Rc<dyn ByteFormatter>) -> Result<Command, String> {
    let bin2 = [bin[0], bin[1]];
    let bin16 = byte_formatter.wrap_bytes(bin2);

    let command_id = bin16 >> 10;
    let command_type = *CODE_ID_MAPPING.get(&command_id).ok_or("invalid command id")?;

    let operand = parse_operand(bin16)?;

    Ok(Command {
        command_type,
        operand,
    })
}

fn parse_operand(bin: u16) -> Result<Operand, String> {
    let operand_type_id = (bin >> 8) & 0b11;
    let value = bin & ((!0) >> 8);

    let operand_tag = OPERAND_ID_MAPPING.get(&operand_type_id).ok_or("invalid operand type id")?;
    let operand = match operand_tag {
        OperandTag::Literal => Operand::Literal(value),
        OperandTag::Register => Operand::Register(*REGISTER_ID_MAPPING.get(&value).ok_or("invalid register id")?)
    };

    Ok(operand)
}

lazy_static! {
    static ref CODE_ID_MAPPING: HashMap<u16, CommandType> = calc_mapping();

}

fn calc_mapping() -> HashMap<u16, CommandType> {
    let mut res = HashMap::new();
    for command_name in CommandType::iter() {
        let code = command_name.get_code();
        res.insert(code, command_name);
    }
    res
}


lazy_static! {
    static ref OPERAND_ID_MAPPING: HashMap<u16, OperandTag> = calc_operand_mapping();
    static ref REGISTER_ID_MAPPING: HashMap<u16, RegisterName> = calc_register_name_mapping();
}

fn calc_operand_mapping() -> HashMap<u16, OperandTag> {
    let mut res = HashMap::new();
    for operand in [OperandTag::Register, OperandTag::Literal] {
        res.insert(operand.get_id(), operand);
    }
    res
}

fn calc_register_name_mapping() -> HashMap<u16, RegisterName> {
    let mut res = HashMap::new();
    for operand in RegisterName::iter() {
        res.insert(operand.get_id(), operand);
    }
    res
}

struct OperandId(u16);

impl From<OperandId> for OperandTag {
    fn from(value: OperandId) -> Self {
        *OPERAND_ID_MAPPING.get(&value.0).unwrap()
    }
}