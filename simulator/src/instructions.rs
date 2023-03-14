use crate::types::{InstructionToken, InstructionType, Opcode};
use std::convert::TryFrom;
use std::io::BufRead;
pub struct Instructions {
    reader: std::io::BufReader<std::fs::File>,
    buffer: String,
}

impl Instructions {
    pub fn new(path_to_file: String) -> Instructions {
        Instructions {
            reader: std::io::BufReader::new(
                std::fs::File::open(path_to_file).expect("File not found"),
            ),
            buffer: String::with_capacity(250),
        }
    }
}

impl Iterator for Instructions {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        let line: Result<usize, std::io::Error> = self.reader.read_line(&mut self.buffer);
        match line {
            Ok(0) => return None,
            Ok(_) => {
                let instruction_token: Option<u32> = parse_instruction(&self.buffer);
                return Some(instruction_token);
            }
            Err(_) => panic!("Error reading file"),
        }
    }
}

impl Opcode {
    pub fn from_u8(value: u8) -> Opcode {
        return match value {
            0x00 => Opcode::Add,
            0x01 => Opcode::Subtract,
            0x02 => Opcode::Multiply,
            0x03 => Opcode::Divide,
            0x04 => Opcode::And,
            0x05 => Opcode::Or,
            0x06 => Opcode::SetIfLessThan,
            0x07 => Opcode::SetIfEqual,
            0x08 => Opcode::ShiftLeft,
            0x09 => Opcode::ShiftRightLogical,
            0x0A => Opcode::ShiftRightArithmetic,
            0x0B => Opcode::SetLower,
            0x0C => Opcode::SetUpper,
            0x0D => Opcode::LoadWord,
            0x0E => Opcode::SaveWord,
            0x0F => Opcode::Special,
            _ => panic!("Invalid opcode"),
        };
    }
}

impl InstructionType {
    pub fn from_opcode(opcode: &Opcode) -> InstructionType {
        return match opcode {
            Opcode::Add => InstructionType::Register,
            Opcode::Subtract => InstructionType::Register,
            Opcode::Multiply => InstructionType::Register,
            Opcode::Divide => InstructionType::Register,
            Opcode::And => InstructionType::Register,
            Opcode::Or => InstructionType::Register,
            Opcode::SetIfLessThan => InstructionType::Register,
            Opcode::SetIfEqual => InstructionType::Register,
            Opcode::ShiftLeft => InstructionType::Register,
            Opcode::ShiftRightLogical => InstructionType::Register,
            Opcode::ShiftRightArithmetic => InstructionType::Register,
            Opcode::SetLower => InstructionType::Set,
            Opcode::SetUpper => InstructionType::Set,
            Opcode::LoadWord => InstructionType::Memory,
            Opcode::SaveWord => InstructionType::Memory,
            Opcode::Special => InstructionType::Special,
        };
    }
}

fn parse_instruction(instruction: &String) -> Option<u32> {
    let instruction_cleaned: String = instruction.replace(" ", "");
    let instruction_string = instruction_cleaned
        .split("#")
        .next()
        .expect("Couldn't remove comment");
    if instruction_string == "" {
        println!("Empty line");
        return None;
    }
    let instruction: u32 = u32::from_str_radix(instruction_string, 16).expect("Invalid hex number");
    println!("{:#06X}", instruction);
    return Some(instruction);
}
