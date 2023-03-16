use crate::types::{ControlSignals, State};

pub struct StateMachine {
    state: State,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            state: State::InstructionFetch,
        }
    }

    pub fn get_control_signals(&mut self) -> ControlSignals {
        match self.state {
            State::Terminate => ControlSignals {
                decode: false,
                terminate: true,
                address_source: crate::types::AddressSource::ALU,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                upper_register_write: false,
                long_register_write: false,
                read_pc: false,
                write_pc: false,
                write_register_source: crate::types::RegisterSource::Instruction,
                alu_operation: crate::types::AluOperation::Add,
            },
            State::Decode => ControlSignals {
                decode: true,
                terminate: false,
                address_source: crate::types::AddressSource::ALU,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                upper_register_write: false,
                long_register_write: false,
                read_pc: false,
                write_pc: false,
                write_register_source: crate::types::RegisterSource::Instruction,
                alu_operation: crate::types::AluOperation::Add,
            },
            _ => ControlSignals {
                decode: false,
                terminate: false,
                address_source: crate::types::AddressSource::ALU,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                upper_register_write: false,
                long_register_write: false,
                read_pc: false,
                write_pc: false,
                write_register_source: crate::types::RegisterSource::Instruction,
                alu_operation: crate::types::AluOperation::Add,
            },
        }
    }

    pub fn next_state(&mut self, instruction_token: &crate::types::InstructionToken) {
        match self.state {
            State::InstructionFetch => {
                self.state = State::Decode;
            }
            State::Decode => match instruction_token.opcode {
                _ => {
                    self.state = State::Terminate;
                }
            },
            State::Terminate => {
                println!("Program terminated, memory and registers dumped");
            }
        }
    }
}
