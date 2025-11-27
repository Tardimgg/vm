mod command;
mod memory;
mod register;
mod operand;
mod byte_formatter;
mod common;
mod decoder;
mod program_counter;
mod file_loaders;
mod vm;
mod encoder;

use std::borrow::Cow;
use std::cell::RefCell;
use std::{env, fs, io};
use crate::byte_formatter::byte_formatter::ByteFormatter;
use crate::common::default_error::DefaultError;
use crate::vm::{init_vm, next_step, VmState};
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use std::error::Error;
use std::fmt::format;
use std::io::{stdout, Write};
use std::ops::{Add, Deref};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use lazy_static::lazy_static;
use ratatui::backend::Backend;
use ratatui::prelude::{Color, Constraint, Direction, Layout, Line, Modifier, Rect, Span, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use crate::byte_formatter::little_endian_formatter::LittleEndianFormatter;
use crate::command::{Command, CommandType};
use crate::decoder::asm_decoder::parse_asm;
use crate::encoder::bin_encoder::encode_command;
use crate::file_loaders::load_string_file;
use crate::memory::command_memory::CommandMemory;
use crate::memory::data_memory::DataMemory;
use crate::operand::Operand;
use crate::register::{Register, RegisterName};

fn main() -> Result<(), String> {
    let bytes_formatter = Rc::new(LittleEndianFormatter::default());

    let args: Vec<_> = env::args().collect();
    // let args: Vec<_> = vec!["qwe", "/home/oop/MIREA/мага/1 курс/Разработка программно-аппаратного обеспечения информационных и автоматизированных систем/lab1/bin"];
    // let args: Vec<_> = vec!["qwe", "compile", "/home/oop/MIREA/мага/1 курс/Разработка программно-аппаратного обеспечения информационных и автоматизированных систем/lab1/asm"];
    if args.len() < 2 {
        return Err("invalid params".to_string());
    }


    if args.len() > 1 && args[1] == "compile" {
        let in_file = &args[2];

        let asm = parse_asm(&load_string_file(in_file).default_res()?)?;

        stdout().write_all(&asm.into_iter()
            .map(|v| encode_command(&v, bytes_formatter.clone()))
            .flat_map(|v| v)
            .collect::<Vec<u8>>()).unwrap();

        return Ok(())
    }


    let (command_path, memory_path, dump_path) = if args.len() > 1 && args[1] == "dump" {
        (&args[2], None, args.get(3))
    } else {
        (&args[1], args.get(2), None)
    };

    let command_memory = CommandMemory::load(fs::read(command_path).default_res()?, bytes_formatter.clone());

    let memory_path = memory_path.map(|v| fs::read(v).unwrap());
    let data_memory = memory_path.map(|v| DataMemory::restore(&v));

    let vm_state = init_vm(bytes_formatter, command_memory, data_memory)?;


    color_eyre::install().default_res()?;
    let terminal = ratatui::init();
    let result = run(terminal, vm_state);
    ratatui::restore();

    if result.is_ok() && dump_path.is_some() {
        let dump = result?.data_memory.dump();
        fs::File::create(dump_path.unwrap()).default_res()?.write(&dump).default_res()?;

        Ok(())
    } else {
        result.map(|_| ())
    }
}

fn run(mut terminal: DefaultTerminal, mut vm: VmState) -> Result<VmState, String> {
    let args: Vec<_> = env::args().collect();

    loop {
        terminal.draw(|f| render(f, &vm)).default_res()?;
        match event::read().default_res()? {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char(' ') {
                    if !next_step(&mut vm)? {
                        // break Ok(())
                    }
                }
                if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    break Ok(vm)
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, vm_state: &VmState) {
    ui(frame, vm_state)
    // frame.render_widget("hello world", frame.area());
}


fn ui(f: &mut Frame, vm_state: &VmState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Заголовок
            Constraint::Min(10),    // Основная область
            // Constraint::Length(3),  // Статус/ввод
        ])
        .split(f.size());

    // Заголовок
    let header = Paragraph::new("Press space to continue")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Основная область
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Код
            Constraint::Percentage(30), // Регистры и стек
            Constraint::Percentage(30), // Память и вывод
        ])
        .split(chunks[1]);

    // Код программы
    render_code(f, vm_state, main_chunks[0]);

    // Регистры и стек
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(100), // Регистры
            // Constraint::Percentage(50), // Стек
        ])
        .split(main_chunks[1]);

    // render_registers(f, vm_state, right_chunks[0]);
    render_registers(f, vm_state, main_chunks[1]);
    // render_stack(f, vm_state, right_chunks[1]);

    // Память и вывод
    let output_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // Память
            Constraint::Percentage(60), // Вывод
        ])
        .split(main_chunks[2]);

    // render_memory(f, vm_state, output_chunks[0]);
    render_memory(f, vm_state, main_chunks[2]);
    // render_output(f, vm_state, output_chunks[1]);

    // Статус/ввод
    // render_status(f, vm_state, chunks[2]);
}

impl From<Command> for Cow<'_, str> {
    fn from(value: Command) -> Self {
        let command_name: &str = value.command_type.into();
        let operand_str: &str = match value.operand {
            Operand::Literal(v) => &v.to_string(),
            Operand::Register(r) => r.into()
        };
        Cow::from(format!("{} {}", command_name, operand_str))
    }
}

fn render_code(f: &mut Frame, vm_state: &VmState, area: Rect) {
    let mut sorted_commands = vm_state.command_memory.get_all();
    sorted_commands.sort_by_key(|v| v.0);

    let selected = sorted_commands.iter().enumerate().find(|v| v.1.0 == vm_state.pc.pc).map(|v| v.0);


    let code_items: Vec<ListItem> = sorted_commands
        .into_iter()
        .map(|(address, command)| {
            let mut style = Style::default();

            if address == vm_state.pc.pc {
                style = style.bg(Color::DarkGray).add_modifier(Modifier::BOLD);
            }

            let formatter = vm_state.bytes_formatter.clone();
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:04X}; ", address), Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:04}: ", address), Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:04X}; ", formatter.clone().wrap_bytes(encode_command(&command, formatter))), style),
                Span::styled(command.clone(), style),
            ]))
        })
        .collect();

    let code_list = List::new(code_items)
        .scroll_padding(2)
        .block(
            Block::default()
                .title(" Code ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let mut list_state = ListState::default().with_selected(selected);

    f.render_stateful_widget(code_list, area, &mut list_state);
}

fn render_registers(f: &mut Frame, vm_state: &VmState, area: Rect) {
    let mut registers_text = vec![];

    let mut sorted_registers = vm_state.registers.get_all().iter().map(|v| (v.0, v.1)).collect::<Vec<_>>();
    sorted_registers.sort_by_key(|(name, registry)| (*name).get_id());

    for (name, value) in sorted_registers.iter() {
        let register_name: &str = (*name).into();
        registers_text.push(Line::from(vec![
            Span::styled(format!("{:3}: ", register_name), Style::default().fg(Color::Yellow)),
            Span::styled(format!("{:04X} ({})", value.as_u16(), value.as_u16()), Style::default().fg(Color::Green)),
        ]));
    }

    let registers = Paragraph::new(registers_text)
        .block(Block::default().title(" Registers ").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(registers, area);
}

fn render_stack(f: &mut Frame, vm_state: &VmState, area: Rect) {
    // let stack_items: Vec<ListItem> = app
    //     .stack
    //     .iter()
    //     .rev()
    //     .enumerate()
    //     .map(|(i, val)| {
    //         ListItem::new(format!("[{:02}]: {:08X} ({})",
    //                               app.stack.len() - i - 1, val, val))
    //     })
    //     .collect();

    let stack_list = List::new(vec![] as Vec<ListItem>)
        .block(Block::default().title(" Stack ").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(stack_list, area);
}

fn get_value(operand: &Operand, vm_state: &VmState) -> u16 {
    match operand {
        Operand::Literal(v) => *v,
        Operand::Register(register_name) => {
            vm_state.registers.get(*register_name).unwrap().as_u16()
        }
    }
}

lazy_static! {
    static ref LAST_RENDERED_MEMORY_ADDRESS: Arc<Mutex<Option<u16>>> = Arc::new(Mutex::new(None));
}

fn render_memory(f: &mut Frame, vm_state: &VmState, area: Rect) {

    let current_command = vm_state.command_memory.get(vm_state.pc.pc).unwrap_or(None);

    let memory_address_o = if current_command.is_none() {
        None
    } else {
        let current_command = current_command.unwrap();
        match current_command.command_type {
            CommandType::LAC => Some(get_value(&current_command.operand, vm_state)),
            CommandType::DAC => Some(get_value(&current_command.operand, vm_state)),
            _ => None
        }
    };

    let memory_address = if let Some(v) = memory_address_o {
        v
    } else {
        LAST_RENDERED_MEMORY_ADDRESS.clone().lock().unwrap().unwrap_or(0u16)
    };

    let mut memory_lines = Vec::new();

    let min_address = i32::max(0, memory_address as i32 - 50) as u16;
    let max_address = u32::min(u16::MAX as u32, memory_address as u32 + 50) as u16;

    let mut used_index = None;
    let mut result_list_index = 0;

    for current_address in min_address..=max_address {
        // let offset123 = app.memory_offset + offset * 16;
        // if offset123 >= app.memory.len() {
        //     break;
        // }
        let mut style = Style::default();


        if current_address == memory_address {
            used_index = Some(result_list_index);
            style = style.bg(Color::DarkGray).add_modifier(Modifier::BOLD);
        }


        result_list_index += 1;

        let mut line_spans = vec![
            Span::styled(format!("{:04X}; ", current_address), Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:04}: ", current_address), Style::default().fg(Color::DarkGray)),
        ];

        line_spans.push(Span::styled(format!("{:02X} ", vm_state.data_memory.get(current_address)), style));
        line_spans.push(Span::styled(format!("{:02} ", vm_state.data_memory.get(current_address)), style));


        // for col in 0..16 {
        //     let idx = offset123 + col;
        //     if idx < app.memory.len() {
        //         line_spans.push(Span::raw(format!("{:02X} ", app.memory[idx])));
        //     }
        // }

        memory_lines.push(ListItem::new(Line::from(line_spans)));
    }

    let memory_list = List::new(memory_lines)
        .scroll_padding(2)
        .block(
            Block::default()
                .title(" Memory ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    // let memory = Paragraph::new(memory_lines)
    //     .block(Block::default().title(" Memory ").borders(Borders::ALL))
    //     .style(Style::default().fg(Color::White));


    if let Some(used) = used_index {
        *LAST_RENDERED_MEMORY_ADDRESS.lock().unwrap() = Some(used as u16);
    }

    f.render_stateful_widget(memory_list, area, &mut ListState::default().with_selected(used_index));
}

fn render_output(f: &mut Frame, vm_state: &VmState, area: Rect) {
    // let output_text: Vec<Line> = app
    //     .output
    //     .iter()
    //     .rev()
    //     .take(area.height as usize - 2)
    //     .map(|s| Line::from(s.clone()))
    //     .collect();

    let output = Paragraph::new(vec![])
        .block(Block::default().title(" Output ").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(output, area);
}

fn render_status(f: &mut Frame, vm_state: &VmState, area: Rect) {
    // let status_text = if app.input_mode {
    //     format!("Command: {}_", app.input)
    // } else {
    //     let status = if app.is_running {
    //         if app.is_paused {
    //             "PAUSED"
    //         } else {
    //             "RUNNING"
    //         }
    //     } else {
    //         "STOPPED"
    //     };
    //
    //     format!(
    //         " Status: {} | [r]un [s]tep [p]ause [c]lear [i]nput [q]uit",
    //         status
    //     )
    // };

    // let style = if app.is_running {
        let style = Style::default().fg(Color::Green);
    // } else if app.is_paused {
    //     Style::default().fg(Color::Yellow)
    // } else {
    //     Style::default().fg(Color::Red)
    // };

    let status = Paragraph::new("RUNNING")
        .block(Block::default().borders(Borders::ALL))
        .style(style);

    f.render_widget(status, area);
}
