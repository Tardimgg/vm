use std::rc::Rc;
use crate::byte_formatter::byte_formatter::ByteFormatter;
use crate::byte_formatter::little_endian_formatter::LittleEndianFormatter;
use crate::command::CommandType;
use crate::common::default_error::DefaultError;
use crate::decoder::asm_decoder::parse_asm;
use crate::file_loaders::load_string_file;
use crate::memory::command_memory::CommandMemory;
use crate::memory::data_memory::DataMemory;
use crate::memory::register_memory::RegisterMemory;
use crate::operand::Operand;
use crate::program_counter::ProgramCounter;
use crate::register::RegisterName;

pub struct VmState {
    pub command_memory: CommandMemory,
    pub data_memory: DataMemory,
    pub registers: RegisterMemory,
    pub pc: ProgramCounter,
    pub bytes_formatter: Rc<dyn ByteFormatter>
}

pub fn init_vm(bytes_formatter: Rc<dyn ByteFormatter>, command_memory: CommandMemory, data_memory: Option<DataMemory>) -> Result<VmState, String> {

    // let file = parse_bin(&load_file("bin").default_res()?, bytes_formatter.clone().deref())?;
    // let commands = parse_asm(&load_string_file(data_path).default_res()?)?;

    // let command_memory = CommandMemory::load(commands).expect("invalid asm");
    // let data_memory = DataMemory::default();
    let registers = RegisterMemory::new();

    let pc = ProgramCounter::default();

    Ok(VmState {
        command_memory,
        data_memory: data_memory.unwrap_or(DataMemory::default()),
        registers,
        pc,
        bytes_formatter,
    })
}

pub fn next_step(state: &mut VmState) -> Result<bool, String> {
    let command_memory = &state.command_memory;
    let data_memory = &mut state.data_memory;
    let registers = &mut state.registers;
    let bytes_formatter = &mut state.bytes_formatter;

    let pc = &mut state.pc;

    pc.reset_flag();
    let command_o = command_memory.get(pc.pc).unwrap_or(None);
    let command = if let Some(v) = command_o {
        v
    } else {
        return Ok(false);
    };


    let value = match command.operand {
        Operand::Literal(v) => {v}
        Operand::Register(register_id) => registers.get_mut(register_id).expect("invalid register ref").as_u16()
    };


    match command.command_type {
        CommandType::NOP => {}
        CommandType::JMP => {
            pc.set(value);
        }
        CommandType::ADD => {
            let acc = registers.get_mut(RegisterName::Acc).unwrap();
            acc.put_u16(acc.as_u16().cast_signed().wrapping_add(value.cast_signed()).cast_unsigned());
        }
        CommandType::LAC => {
            let acc = registers.get_mut(RegisterName::Acc).unwrap();
            let bytes = [data_memory.get(value), data_memory.get(value + 1)];
            acc.put_u16(bytes_formatter.wrap_bytes(bytes))
        }
        CommandType::DAC => {
            let acc = registers.get_mut(RegisterName::Acc).unwrap();
            let bytes = bytes_formatter.unwrap_bytes(acc.as_u16());
            data_memory.put_bytes(value, &bytes);
        }
        CommandType::SUB => {
            let acc = registers.get_mut(RegisterName::Acc).unwrap();
            acc.put_u16(acc.as_u16() - value);
        }
        CommandType::CMP => {
            let acc = registers.get_mut(RegisterName::Acc).unwrap();
            let res = acc.as_u16().cmp(&value) as i16;
            acc.put_u16(res.cast_unsigned());
        }
        CommandType::MOV => {
            let acc = registers.get_mut(RegisterName::Acc).unwrap();;
            acc.put_u16(value);
        }
        CommandType::LRG => {
            let acc_value = registers.get_mut(RegisterName::Acc).unwrap().as_u16();
            match command.operand {
                Operand::Literal(_) => { return Err("invalid lrg command".to_string()); }
                Operand::Register(target) => {
                    registers.get_mut(target).expect("invalid register").put_u16(acc_value);
                }
            }
        }
        CommandType::JMPG => {
            let acc_value = registers.get_mut(RegisterName::Acc).unwrap().as_u16();
            if acc_value == 1 {
                pc.set(value);
            }
        }
        CommandType::JMPNG => {
            let acc_value = registers.get_mut(RegisterName::Acc).unwrap().as_u16();
            if acc_value != 1 {
                pc.set(value);
            }
        }
    }
    if !pc.changed {
        pc.set(pc.pc + 2)
    }
    Ok(true)
}