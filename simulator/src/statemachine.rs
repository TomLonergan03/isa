use crate::types::{
    AddressSource, AluOperation, AluSource, ControlSignals, InstructionToken, Opcode,
    RegisterWriteSource, State,
};
use log::{error, info, trace};

pub struct StateMachine {
    state: State,
    opcode: Opcode,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        trace!("Initialising state machine in PcRead state");
        StateMachine {
            state: State::PcRead,
            opcode: Opcode::Invalid,
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
                register_write_source: RegisterWriteSource::InstructionByte2,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::PcRead => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::InstructionByte2,
                write_upper: false,
                write_long: false,
                read_pc: true,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::InstructionFetch => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: true,
                memory_write: false,
                instruction_register_write: true,
                register_write: false,
                register_write_source: RegisterWriteSource::InstructionByte2,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Add,
                alu_source: AluSource::Constant1,
                process_special: false,
            },
            State::Decode => ControlSignals {
                terminate: false,
                decode: true,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::Alu,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: true,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::SetLower => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: true,
                register_write_source: RegisterWriteSource::InstructionByte2,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::SetUpper => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: true,
                register_write_source: RegisterWriteSource::InstructionByte2,
                write_upper: true,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::ArithmeticOperation => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::Alu,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::from_opcode(&self.opcode),
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::SetIf => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::InstructionByte2,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Subtract,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::Memory => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::InstructionByte2,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Add,
                alu_source: AluSource::MemoryOffset,
                process_special: false,
            },
            State::ArithmeticWriteBack => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: true,
                register_write_source: RegisterWriteSource::Alu,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::SetIfLess => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: true,
                register_write_source: RegisterWriteSource::AluNegative,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::SetIfEqual => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::ProgramCounter,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: true,
                register_write_source: RegisterWriteSource::AluZero,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::MemoryRead => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::Alu,
                memory_read: true,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::Alu,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::MemoryWrite => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::Alu,
                memory_read: false,
                memory_write: true,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::Alu,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::MemoryReadRegisterWriteback => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::Alu,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: true,
                register_write_source: RegisterWriteSource::Memory,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::SetPcTest => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::Alu,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::InstructionByte2,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Subtract,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::SetPcWriteback => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::Alu,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: true,
                register_write_source: RegisterWriteSource::InstructionNibble2,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: false,
            },
            State::Special => ControlSignals {
                terminate: false,
                decode: false,
                address_source: AddressSource::Alu,
                memory_read: false,
                memory_write: false,
                instruction_register_write: false,
                register_write: false,
                register_write_source: RegisterWriteSource::Alu,
                write_upper: false,
                write_long: false,
                read_pc: false,
                write_pc: false,
                alu_operation: AluOperation::Inactive,
                alu_source: AluSource::Register,
                process_special: true,
            },
            // _ => {
            //     error!("Unimplemented state for control signals: {:?}", self.state);
            //     ControlSignals {
            //         terminate: true,
            //         decode: false,
            //         address_source: AddressSource::ProgramCounter,
            //         memory_read: false,
            //         memory_write: false,
            //         instruction_register_write: false,
            //         register_write: false,
            //         register_write_source: RegisterWriteSource::Instruction,
            //         write_upper: false,
            //         write_long: false,
            //         read_pc: false,
            //         write_pc: false,
            //         alu_operation: AluOperation::Inactive,
            //         alu_source: AluSource::Register,
            //         process_syscall: false,
            //     }
            // }
        }
    }

    pub fn next_state(
        &mut self,
        instruction_token: &crate::types::InstructionToken,
        alu_zero: bool,
    ) {
        match self.state {
            State::PcRead => self.state = State::InstructionFetch,
            State::InstructionFetch => self.state = State::Decode,
            State::Decode => self.decode(instruction_token),
            State::SetLower => self.state = State::PcRead,
            State::SetUpper => self.state = State::PcRead,
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
                Opcode::LoadWord => self.state = State::MemoryRead,
                Opcode::SaveWord => self.state = State::MemoryWrite,
                _ => {
                    error!("In Memory state with non Memory opcode. You should not be here");
                    self.state = State::Terminate;
                }
            },
            State::Special => self.state = State::PcRead,
            State::ArithmeticWriteBack => self.state = State::PcRead,
            State::SetIfLess => self.state = State::PcRead,
            State::SetIfEqual => self.state = State::PcRead,
            State::MemoryRead => self.state = State::MemoryReadRegisterWriteback,
            State::MemoryWrite => self.state = State::PcRead,
            State::MemoryReadRegisterWriteback => self.state = State::PcRead,
            State::SetPcTest => {
                if self.opcode == Opcode::SetPcIf && alu_zero {
                    self.state = State::SetPcWriteback;
                } else if self.opcode == Opcode::SetPcIfNot && !alu_zero {
                    self.state = State::SetPcWriteback;
                } else {
                    self.state = State::PcRead;
                }
            }
            State::SetPcWriteback => self.state = State::PcRead,
            State::Terminate => {
                info!("Program terminated, memory and registers dumped");
            }
        }
        if self.state == State::PcRead {
            info!("New instruction")
        }
        trace!("Entering state: {:?}", self.state);
    }

    fn decode(&mut self, instruction_token: &InstructionToken) {
        self.opcode = instruction_token.opcode.clone();
        info!("Decoding instruction: {:?}", instruction_token.opcode);
        match instruction_token.opcode {
            Opcode::SetLower => self.state = State::SetLower,
            Opcode::SetUpper => self.state = State::SetUpper,
            Opcode::SetIfLess => self.state = State::SetIf,
            Opcode::SetIfEqual => self.state = State::SetIf,
            Opcode::LoadWord => self.state = State::Memory,
            Opcode::SaveWord => self.state = State::Memory,
            Opcode::Special => self.state = State::Special,
            Opcode::SetPcIf => self.state = State::SetPcTest,
            Opcode::SetPcIfNot => self.state = State::SetPcTest,
            Opcode::Invalid => {
                self.state = State::Terminate;
                error!("Invalid opcode encountered, terminating program");
            }
            _ => self.state = State::ArithmeticOperation,
        }
    }
}
