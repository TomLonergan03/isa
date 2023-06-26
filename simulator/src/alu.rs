use log::{error, trace};

use crate::types::{AluOperation, AluOutput};

/// The ALU for an AYU processor
pub struct Alu {}

impl Alu {
    pub fn new() -> Alu {
        trace!("Initializing ALU");
        return Alu {};
    }

    /// Carry out an operation on 2 values
    pub fn execute_operation(
        &self,
        source_a: u16,
        source_b: u16,
        operation: &AluOperation,
    ) -> AluOutput {
        match operation {
            AluOperation::Add => {
                let result: u32 = (source_a.wrapping_add(source_b)) as u32;
                let zero: bool = result == 0;
                let negative: bool = match result & 0b1000000000000000 {
                    1 => true,
                    _ => false,
                };
                return AluOutput {
                    result,
                    zero,
                    negative,
                };
            }
            AluOperation::Subtract => {
                let source_b: u16 = !source_b + 1;
                let result: u32 = (source_a.wrapping_add(source_b)) as u32;
                let zero: bool = result == 0;
                let negative: bool = match result & 0b1000000000000000 {
                    0b1000000000000000 => true,
                    _ => false,
                };
                return AluOutput {
                    result,
                    zero,
                    negative,
                };
            }
            AluOperation::And => {
                let result: u32 = (source_a & source_b) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                return AluOutput {
                    result,
                    zero,
                    negative,
                };
            }
            AluOperation::Or => {
                let result: u32 = (source_a | source_b) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                return AluOutput {
                    result,
                    zero,
                    negative,
                };
            }
            AluOperation::ShiftLeft => {
                let result: u32 = (source_a << source_b) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                return AluOutput {
                    result,
                    zero,
                    negative,
                };
            }
            AluOperation::ShiftRightLogical => {
                let result: u32 = (source_a >> source_b) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                return AluOutput {
                    result,
                    zero,
                    negative,
                };
            }
            AluOperation::ShiftRightArithmetic => {
                let result: u32 = if (source_a & 0b1000000000000000) == 0b1000000000000000 {
                    // Fill right shift 0 bits with 1s
                    ((source_a) >> source_b) as u32 | (0xFFFF << (16 - source_b))
                } else {
                    ((source_a) >> source_b) as u32
                };
                let zero: bool = result == 0;
                let negative: bool = false;
                return AluOutput {
                    result,
                    zero,
                    negative,
                };
            }
            AluOperation::Inactive => {
                error!("ALU operation was inactive");
                return AluOutput {
                    result: 0,
                    zero: false,
                    negative: false,
                };
            }
        }
    }
}
