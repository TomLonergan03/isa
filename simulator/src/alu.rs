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
        source_1: u16,
        source_2: u16,
        operation: &AluOperation,
    ) -> AluOutput {
        match operation {
            AluOperation::Add => {
                let result: u32 = (source_1 + source_2) as u32;
                let result: u16 = (result & 0xFFFFFFFF) as u16;
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
                let result: u64 = (source_1 as u16 - source_2 as u16) as u64;
                let result: u32 = (result & 0xFFFFFFFF) as u32;
                let zero: bool = result == 0;
                let negative: bool = match result & 0b1000000000000000 {
                    1 => true,
                    _ => false,
                };
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::And => {
                let result: u32 = (source_1 & source_2) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::Or => {
                let result: u32 = (source_1 | source_2) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::ShiftLeft => {
                let result: u32 = (source_1 << source_2) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::ShiftRightLogical => {
                let result: u32 = (source_1 >> source_2) as u32;
                let zero: bool = result == 0;
                let negative: bool = false;
                AluOutput {
                    result,
                    zero,
                    negative,
                }
            }
            AluOperation::ShiftRightArithmetic => {
                let result: u32 = ((source_1 as i16) >> source_2) as u32;
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
