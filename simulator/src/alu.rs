use log::{error, trace};

use crate::types::{AluOperation, AluOutput};

/// The ALU for an AYU processor
pub struct Alu {}

impl Alu {
    pub fn new() -> Alu {
        trace!("Initializing ALU");
        Alu {}
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
                let result: u16 = source_a.wrapping_add(source_b);
                let zero: bool = result == 0;
                let negative: bool = matches!(result & 0b1000000000000000, 1);
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::Subtract => {
                let source_b: u16 = !source_b + 1;
                let result: u16 = source_a.wrapping_add(source_b);
                let zero: bool = result == 0;
                let negative: bool = matches!(result & 0b1000000000000000, 1);
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::And => {
                let result: u16 = source_a & source_b;
                let zero: bool = result == 0;
                let negative: bool = matches!(result & 0b1000000000000000, 1);
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::Or => {
                let result: u16 = source_a | source_b;
                let zero: bool = result == 0;
                let negative: bool = matches!(result & 0b1000000000000000, 1);
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::ShiftLeft => {
                let result: u16 = source_a.checked_shl(source_b as u32).unwrap_or(0);
                let zero: bool = result == 0;
                let negative: bool = matches!(result & 0b1000000000000000, 1);
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::ShiftRightLogical => {
                let result: u16 = source_a.checked_shr(source_b as u32).unwrap_or(0);
                let zero: bool = result == 0;
                let negative: bool = matches!(result & 0b1000000000000000, 1);
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::ShiftRightArithmetic => {
                let shift_result: u16 = source_a.checked_shr(source_b as u32).unwrap_or(0);
                let leading_zeroes = shift_result.leading_zeros();
                let mask = ((0b1111111111111111 << leading_zeroes) & 0xFFFF) as u16;
                let result: u16 = shift_result | mask;
                let negative: bool = matches!(source_a & 0b1000000000000000, 1);
                let zero: bool = result == 0;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::Inactive => {
                error!("ALU operation was inactive");
                AluOutput {
                    result: 0,
                    zero: false,
                    negative: false,
                }
            }
        }
    }
}
