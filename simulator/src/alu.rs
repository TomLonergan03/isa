use log::{error, trace};

use crate::types::{AluOperation, AluOutput};

pub struct Alu {}

impl Alu {
    pub fn new() -> Alu {
        trace!("Initializing ALU");
        return Alu {};
    }
    pub fn execute_operation(
        &self,
        source_a: u16,
        source_b: u16,
        operation: &AluOperation,
    ) -> AluOutput {
        match operation {
            AluOperation::Add => {
                let result: u32 = (source_a + source_b) as u32;
                let zero: bool = result == 0;
                let negative: bool = match result & 0b1000000000000000 {
                    1 => true,
                    _ => false,
                };
                let result: u32 = result as u32;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::Subtract => {
                // if there would be (integer) underflow convert to 2s complement
                let mut source_a = source_a;
                let mut source_b = source_b;
                if source_a < source_b {
                    source_b = source_b - source_a;
                    source_a = 0xFFFF;
                }
                let result: u32 = (source_a - source_b) as u32;
                let zero: bool = result == 0;
                let negative: bool = match result & 0b1000000000000000 {
                    0b1000000000000000 => true,
                    _ => false,
                };
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::And => {
                let result: u32 = (source_a & source_b) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::Or => {
                let result: u32 = (source_a | source_b) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::ShiftLeft => {
                let result: u32 = (source_a << source_b) as u32;
                println!("{} << {} = {}", source_a, source_b, result);
                let zero: bool = result == 0;
                let negative: bool = false;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::ShiftRightLogical => {
                let result: u32 = (source_a >> source_b) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::ShiftRightArithmetic => {
                let result: u32 = match (source_a & 0b1000000000000000) == 0b1000000000000000 {
                    true => ((source_a) >> source_b) as u32 | (0xFFFF << (16 - source_b)),
                    false => ((source_a) >> source_b) as u32,
                };
                let zero: bool = result == 0;
                let negative: bool = false;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            _ => {
                error!("Unimplemented AluOperation {:?}", operation);
                return AluOutput {
                    result: 0,
                    zero: false,
                    negative: false,
                };
            }
        }
    }
}
