use std::rc::Rc;
use enum_tags_traits::TaggedEnum;
use crate::byte_formatter::byte_formatter::ByteFormatter;
use crate::command::Command;
use crate::operand::Operand;

pub fn encode_command(command: &Command, byte_formatter: Rc<dyn ByteFormatter>) -> [u8; 2] {
    let cmd_type = command.command_type.get_code();
    let operand_type = command.operand.tag().get_id();
    let operand_value = match command.operand {
        Operand::Literal(literal) => literal,
        Operand::Register(name) => name.get_id()
    };

    let mut command = (cmd_type & 0b111111) << 10;
    command += (operand_type & 0b11) << 8;
    command += operand_value & 0xff;

    byte_formatter.unwrap_bytes(command)
}