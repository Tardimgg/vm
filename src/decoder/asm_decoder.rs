use crate::command::{Command, CommandType};
use crate::common::default_error::DefaultError;
use crate::operand::Operand;
use crate::register::RegisterName;
use std::str::FromStr;

pub fn parse_asm(lines: &[String]) -> Result<Vec<Command>, String> {
    let mut res = Vec::new();
    for line in lines {
        let comment_index = line.find("//");

        let filtered_line = if let Some(index) = comment_index {
            &line[..index]
        } else {
            &line
        };

        if filtered_line.is_empty() {
            continue;
        }

        let command = filtered_line.split(" ").collect::<Vec<&str>>();
        let command_type = CommandType::from_str(command[0]).default_res()?;
        let operand = parse_operand(command[1])?;

        res.push(Command {
            command_type,
            operand,
        })
    }

    Ok(res)
}

fn parse_operand(data: &str) -> Result<Operand, String> {
    if let Ok(number) = u16::from_str(data) {
        Ok(Operand::Literal(number))
    } else {
        Ok(Operand::Register(RegisterName::from_str(data).default_res()?))
    }
}