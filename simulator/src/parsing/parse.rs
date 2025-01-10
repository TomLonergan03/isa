use crate::types::Instruction;

pub trait Parse {
    fn parse_instruction(instruction: &str) -> Option<Instruction>;
}
