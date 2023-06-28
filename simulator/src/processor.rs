use crate::alu;
use crate::instructions::parse_instruction;
use crate::statemachine::StateMachine;
use crate::types::{
    AddressSource, AluOperation, AluOutput, AluSource, ControlSignals, InstructionToken,
    InstructionType, Opcode, PipelineRegisters, RegisterWriteSource, RegisterWriteTarget, RunState,
};
use log::{debug, error, info, trace};
use std::fs::File;
use std::io::Write;
use time::OffsetDateTime;

/// An AYU processor simulation
pub struct Processor {
    alu: alu::Alu,
    clock_cycle: u64,
    registers: [u16; 16],
    memory: [u16; 65536],
    instruction_register: u16,
    instruction_token: InstructionToken,
    control_signals: ControlSignals,
    state_machine: StateMachine,
    pipeline_registers: PipelineRegisters,
    breakpoint: u64,
    dump_to_file: bool,
}

impl Processor {
    /// Create a processor by initialising memory to the contents of a .ayu file,
    /// all registers are 0 so starts executing from first instruction
    pub fn new_from_file(path_to_file: String, breakpoint: u64, dump_to_file: bool) -> Processor {
        let instruction_string: String =
            std::fs::read_to_string(path_to_file).expect("File not found");
        let instruction_array: Vec<u16> = instruction_string
            .split("\n")
            .filter(|x| !x.starts_with("#") && !x.is_empty())
            .map(|x| parse_instruction(x).expect("Invalid instruction"))
            .collect();
        let mut memory_array: [u16; 65536] = [0; 65536];
        for (i, instruction) in instruction_array.iter().enumerate() {
            memory_array[i] = *instruction;
        }
        debug!("Memory contents:");
        memory_array.iter().enumerate().for_each(|(i, x)| match x {
            0 => (),
            _ => debug!("M{:#06X}: {:#06X}", i, x),
        });

        return Processor {
            alu: alu::Alu::new(),
            clock_cycle: 0,
            registers: [0; 16],
            memory: memory_array,
            instruction_register: 0,
            instruction_token: InstructionToken {
                instruction_type: InstructionType::Invalid,
                opcode: Opcode::Invalid,
                nibble_2: 0,
                nibble_3: 0,
                nibble_4: 0,
            },
            control_signals: ControlSignals {
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
                write_register_target: RegisterWriteTarget::Nibble2,
            },
            state_machine: StateMachine::new(),
            pipeline_registers: PipelineRegisters {
                memory_data: 0,
                register_read_b: 0,
                register_read_a: 0,
                alu_output: 0,
                alu_negative: false,
                alu_zero: false,
            },
            breakpoint,
            dump_to_file,
        };
    }

    /// Create a processor by initialising registers and memory to provided arrays,
    /// therefore can be used to load a processor that has already partially executed a program
    pub fn new_from_array(
        register_array: [u16; 16],
        memory_array: [u16; 65536],
        dump_to_file: bool,
    ) -> Processor {
        debug!("Register contents:");
        register_array
            .iter()
            .enumerate()
            .for_each(|(i, register)| debug!("R{:#02X}: {:#06X}", i, register));

        debug!("Memory contents:");
        memory_array.iter().enumerate().for_each(|(i, x)| match x {
            0 => (),
            _ => debug!("M{:#06X}: {:#06X}", i, x),
        });
        return Processor {
            alu: alu::Alu::new(),
            clock_cycle: 0,
            registers: register_array,
            memory: memory_array,
            instruction_register: 0,
            instruction_token: InstructionToken {
                instruction_type: InstructionType::Invalid,
                opcode: Opcode::Invalid,
                nibble_2: 0,
                nibble_3: 0,
                nibble_4: 0,
            },
            control_signals: ControlSignals {
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
                write_register_target: RegisterWriteTarget::Nibble2,
            },
            state_machine: StateMachine::new(),
            pipeline_registers: PipelineRegisters {
                memory_data: 0,
                register_read_b: 0,
                register_read_a: 0,
                alu_output: 0,
                alu_negative: false,
                alu_zero: false,
            },
            breakpoint: u64::MAX,
            dump_to_file,
        };
    }

    /// Runs 1 clock cycle, returns whether the processor should continue running for another cycle
    pub fn run(&mut self) -> RunState {
        // state machine shouldn't advance on first cycle
        if self.clock_cycle != 0 {
            self.state_machine
                .next_state(&self.instruction_token, self.pipeline_registers.alu_zero);
        }
        self.control_signals = self.state_machine.get_control_signals();
        // Do ALU op if active
        if self.control_signals.alu_operation != AluOperation::Inactive {
            let source_a: u16 = match self.control_signals.alu_source {
                AluSource::Register => self.pipeline_registers.register_read_a,
                AluSource::Constant1 => 1,
                AluSource::MemoryOffset => self.instruction_token.nibble_2 as u16,
            };
            let source_b: u16 = self.pipeline_registers.register_read_b;
            let alu_result: AluOutput =
                self.alu
                    .execute_operation(source_a, source_b, &self.control_signals.alu_operation);
            trace!(
                "Alu operation {:#06X} {:?} {:#06X} = {:#06X}",
                source_a,
                self.control_signals.alu_operation,
                source_b,
                alu_result.result
            );
            self.pipeline_registers.alu_output = alu_result.result;
            self.pipeline_registers.alu_zero = alu_result.zero;
            self.pipeline_registers.alu_negative = alu_result.negative;
        }
        if self.control_signals.terminate {
            info!("Terminating processor, dumping core");
            self.coredump(self.dump_to_file);
            return RunState::Stop;
        }
        if self.control_signals.decode {
            debug!("Decoding instruction {:#06X}", self.instruction_register);
            self.instruction_token = Processor::decode_instruction(self.instruction_register);
        }
        if self.control_signals.memory_read {
            let address: u16 = match self.control_signals.address_source {
                AddressSource::ProgramCounter => self.pipeline_registers.register_read_b,
                AddressSource::Alu => (self.pipeline_registers.alu_output & 0xFFFF) as u16,
            };
            let data = self.memory[address as usize];
            self.pipeline_registers.memory_data = data;
            if self.control_signals.instruction_register_write {
                self.instruction_register = data;
            }
            trace!(
                "Read M{:#06X} = {:#06X}",
                address,
                self.pipeline_registers.memory_data
            );
        }
        if self.control_signals.memory_write {
            let address: u16 = match self.control_signals.address_source {
                AddressSource::ProgramCounter => self.pipeline_registers.register_read_b,
                AddressSource::Alu => (self.pipeline_registers.alu_output & 0xFFFF) as u16,
            };
            let data = self.pipeline_registers.register_read_a;
            self.memory[address as usize] = data;
            trace!("Wrote M{:#06X} = {:#06X}", address, data);
        }
        if self.control_signals.register_write || self.control_signals.write_pc {
            let value_to_write: u16 = match self.control_signals.register_write_source {
                RegisterWriteSource::Alu => self.pipeline_registers.alu_output as u16 & 0xFFFF,
                RegisterWriteSource::InstructionByte2 => {
                    (((((self.instruction_token.nibble_3 as u16) << 4) & 0xF0) as u8)
                        + self.instruction_token.nibble_4) as u16
                        & 0xFF
                }
                RegisterWriteSource::AluNegative => self.pipeline_registers.alu_negative as u16,
                RegisterWriteSource::AluZero => self.pipeline_registers.alu_zero as u16,
                RegisterWriteSource::Memory => self.pipeline_registers.memory_data,
                RegisterWriteSource::InstructionNibble2 => self.instruction_token.nibble_2 as u16,
            };
            let register_to_write: usize = match self.control_signals.write_pc {
                true => 1,
                false => match self.control_signals.write_register_target {
                    RegisterWriteTarget::Nibble2 => self.instruction_token.nibble_2 as usize,
                    RegisterWriteTarget::Nibble3 => self.instruction_token.nibble_3 as usize,
                },
            };
            if self.control_signals.write_upper {
                trace!(
                    "Writing {:06X} to upper 8 bits of register {:01X}",
                    value_to_write,
                    register_to_write
                );
                self.registers[register_to_write] |= value_to_write << 8;
            } else {
                trace!(
                    "Writing {:06X} to register {:01X}",
                    value_to_write,
                    register_to_write
                );
                self.registers[register_to_write] = value_to_write;
            }
        }

        // update pipeline registers from register read
        self.pipeline_registers.register_read_b = match self.control_signals.read_pc {
            true => {
                trace!("Reading PC");
                self.registers[1]
            }
            false => self.registers[self.instruction_token.nibble_4 as usize],
        };

        self.pipeline_registers.register_read_a =
            self.registers[self.instruction_token.nibble_3 as usize];

        if self.control_signals.process_special {
            match self.instruction_token.nibble_2 {
                1 => {
                    info!("Reached end of program");
                    self.coredump(self.dump_to_file);
                    return RunState::Stop;
                }
                _ => {
                    error!("Unimplemented special instruction");
                    self.coredump(self.dump_to_file);
                    return RunState::Stop;
                }
            }
        }
        if self.clock_cycle as u64 > self.breakpoint {
            info!("Reached breakpoint");
            self.coredump(self.dump_to_file);
            return RunState::Stop;
        }
        self.clock_cycle += 1;
        return RunState::Continue;
    }

    /// Convert an instruction to an InstructionToken
    fn decode_instruction(instruction: u16) -> InstructionToken {
        let opcode: Opcode =
            Opcode::from_u8(u8::try_from((instruction & 0xF000) >> 12).expect("Invalid byte 1"));
        let nibble_2: u8 = u8::try_from((instruction & 0x0F00) >> 8).expect("Invalid byte 2");
        let nibble_3: u8 = u8::try_from((instruction & 0x00F0) >> 4).expect("Invalid byte 3");
        let nibble_4: u8 = u8::try_from(instruction & 0x000F).expect("Invalid byte 4");
        let instruction_type: InstructionType = InstructionType::from_opcode(&opcode);
        return InstructionToken {
            opcode,
            nibble_2,
            nibble_3,
            nibble_4,
            instruction_type,
        };
    }

    /// Dump the current state of a processor to a file
    ///
    /// Returns current register and memory state as a pair of arrays
    pub fn coredump(&self, write_to_file: bool) -> (Vec<u16>, Vec<u16>) {
        let mut dump = format!("Core dump at time: {:#?}\n", OffsetDateTime::now_utc());
        dump.push_str(format!("Clock cycle: {:#?}\n", self.clock_cycle).as_str());
        dump.push_str("\nRegisters:\n");
        let mut dump_registers = Vec::new();
        for (i, register) in self.registers.iter().enumerate() {
            dump.push_str(format!("R{:#02X}: {:#06X}\n", i, register).as_str());
            dump_registers.push(register.clone());
        }
        dump.push_str("\nMemory:\n");
        let mut dump_memory = Vec::new();
        for (i, memory) in self.memory.iter().enumerate() {
            dump.push_str(format!("M{:#06X}: {:#06X}\n", i, memory).as_str());
            dump_memory.push(memory.clone());
        }
        if write_to_file {
            let mut file = File::create("core.dump").expect("Could not create coredump file");
            file.write_all(dump.as_bytes())
                .expect("Could not write to coredump file");
        }
        return (dump_registers, dump_memory);
    }
}
