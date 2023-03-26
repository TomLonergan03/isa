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
                let result: u16 = source_1 + source_2;
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
