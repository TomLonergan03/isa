use log::debug;

use crate::{parsing::parse::Parse, util::types::Instruction};

pub struct Parser;

impl Parse for Parser {
    fn parse_instruction(instruction: &str) -> Option<Instruction> {
        let instruction_cleaned: String = instruction.replace(' ', "");
        let instruction_string = instruction_cleaned
            .split('#')
            .next()
            .expect("Couldn't remove comment");
        if instruction_string.is_empty() {
            debug!("Read empty line");
            return None;
        }
        let instruction: u16 =
            u16::from_str_radix(instruction_string, 16).expect("Invalid hex number");
        debug!("Read instruction: {:#06X}", instruction);
        Some(instruction)
    }
}
