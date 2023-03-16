use crate::instructions::parse_instruction;
use crate::statemachine::StateMachine;
use crate::types::{
    AddressSource, ControlSignals, InstructionToken, InstructionType, Opcode, PipelineRegisters,
};
use log::{debug, info};
use std::fs::File;
use std::io::Write;
use time::OffsetDateTime;

pub struct Processor {
    clock_cycle: u64,
    registers: [u16; 16],
    memory: [u16; 65536],
    instruction_register: u16,
    instruction_token: InstructionToken,
    control_signals: ControlSignals,
    state_machine: StateMachine,
    pipeline_registers: PipelineRegisters,
}

impl Processor {
    pub fn new(path_to_file: String) -> Processor {
        let instruction_string: String =
            std::fs::read_to_string(path_to_file).expect("File not found");
        let instruction_array: Vec<u16> = instruction_string
            .split("\n")
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
            state_machine: StateMachine::new(),
            pipeline_registers: PipelineRegisters {
                memory_data: 0,
                register_read_a: 0,
                register_read_b: 0,
                alu_output: 0,
            },
        };
    }

    // returns false if the processor should terminate
    pub fn run(&mut self) -> bool {
        self.state_machine.next_state(&self.instruction_token);
        self.control_signals = self.state_machine.get_control_signals();
        self.clock_cycle += 1;
        if self.control_signals.terminate {
            info!("Terminating processor, dumping core");
            self.coredump();
            return false;
        }
        if self.control_signals.decode {
            debug!("Decoding instruction {:#06X}", self.instruction_register);
            self.instruction_token = Processor::decode_instruction(self.instruction_register);
        }
        if self.control_signals.memory_read {
            let address: u16 = match self.control_signals.address_source {
                AddressSource::ALU => u16::try_from(self.pipeline_registers.alu_output & 0xFFFF)
                    .expect("Invalid address for memory read"),
                AddressSource::ProgramCounter => self.pipeline_registers.register_read_a,
            };

            self.pipeline_registers.memory_data = self.memory[address as usize];
        }
        if self.control_signals.instruction_register_write {
            let address: u16 = match self.control_signals.address_source {
                AddressSource::ALU => u16::try_from(self.pipeline_registers.alu_output & 0xFFFF)
                    .expect("Invalid address for memory read"),
                AddressSource::ProgramCounter => self.pipeline_registers.register_read_a,
            };
            self.instruction_register = self.memory[address as usize];
        }
        return true;
    }

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

    fn coredump(&self) {
        let mut file = File::create("core.dump").expect("Could not create coredump file");
        let mut dump = format!("Core dump at time: {:#?}\n", OffsetDateTime::now_utc());
        dump.push_str(format!("Clock cycle: {:#?}\n", self.clock_cycle).as_str());
        dump.push_str("\nRegisters:\n");
        for (i, register) in self.registers.iter().enumerate() {
            dump.push_str(format!("R{:#02X}: {:#06X}\n", i, register).as_str());
        }
        dump.push_str("\nMemory:\n");
        for (i, memory) in self.memory.iter().enumerate() {
            dump.push_str(format!("M{:#06X}: {:#06X}\n", i, memory).as_str());
        }
        file.write_all(dump.as_bytes())
            .expect("Could not write to coredump file");
        info!("Core dumped")
    }
}
