use crate::types::{
    AddressSource, AluOperation, AluSource, ControlSignals, Opcode, RegisterWriteSource, State,
};
use log::{error, info, trace};

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
                terminate: true,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::Instruction,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Add,
                alu_source: AluSource::Register,
            },
            _ => {
                error!("Unimplemented state");
                ControlSignals {
                    terminate: true,
                    decode: false,
                    address_source: AddressSource::ProgramCounter,
                    memory_read: false,
                    memory_write: false,
                    instruction_register_write: false,
                    register_write: false,
                    register_write_source: RegisterWriteSource::Instruction,
                    write_upper: false,
                    write_long: false,
                    read_pc: false,
                    write_pc: false,
                    alu_operation: AluOperation::Add,
                    alu_source: AluSource::Register,
                }
            }
        }
    }

    pub fn next_state(&mut self, instruction_token: &crate::types::InstructionToken) {
        match self.state {
            State::PCRead => self.state = State::InstructionFetch,
            State::InstructionFetch => self.state = State::Decode,
            State::Decode => self.decode(instruction_token),
            State::SetLower => self.state = State::PCRead,
            State::SetUpper => self.state = State::PCRead,
            State::ArithmeticOperation => self.state = State::ArithmeticWriteBack,
            State::SetIf => match instruction_token.opcode {
                Opcode::SetIfLess => self.state = State::SetIfLess,
                Opcode::SetIfEqual => self.state = State::SetIfEqual,
                _ => {
                    error!("In SetIf state with non SetIf opcode. You should not be here");
                    self.state = State::Terminate;
                }
            },
            State::Memory => match instruction_token.opcode {
                Opcode::LoadWord => self.state = State::MemoryLoad,
                Opcode::SaveWord => self.state = State::MemorySave,
                _ => {
                    error!("In Memory state with non Memory opcode. You should not be here");
                    self.state = State::Terminate;
                }
            },
            State::Special => self.state = State::PCRead,
            State::ArithmeticWriteBack => self.state = State::PCRead,
            State::SetIfLess => self.state = State::PCRead,
            State::SetIfEqual => self.state = State::PCRead,
            State::MemoryLoad => self.state = State::MemoryLoadWriteBack,
            State::MemorySave => self.state = State::PCRead,
            State::MemoryLoadWriteBack => self.state = State::PCRead,
            State::Terminate => {
                info!("Program terminated, memory and registers dumped");
            }
        }
        trace!("Entering state: {:?}", self.state);
    }

    fn decode(&mut self, instruction_token: &crate::types::InstructionToken) {
        match instruction_token.opcode {
            Opcode::SetLower => self.state = State::SetLower,
            Opcode::SetUpper => self.state = State::SetUpper,
            Opcode::SetIfLess => self.state = State::SetIf,
            Opcode::SetIfEqual => self.state = State::SetIf,
            Opcode::LoadWord => self.state = State::Memory,
            Opcode::SaveWord => self.state = State::Memory,
            Opcode::Special => self.state = State::Special,
            _ => self.state = State::ArithmeticOperation,
        }
    }
}
