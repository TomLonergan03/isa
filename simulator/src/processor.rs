use crate::instructions::parse_instruction;
use crate::statemachine::StateMachine;
use crate::types::{ControlSignals, InstructionToken, InstructionType, Opcode};

pub struct Processor {
    clock_cycle: u64,
    registers: [u16; 16],
    memory: [u16; 4096],
    instruction_register: u16,
    control_signals: ControlSignals,
    state_machine: StateMachine,
}

impl Processor {
    pub fn new(path_to_file: String) -> Processor {
        let instruction_string: String =
            std::fs::read_to_string(path_to_file).expect("File not found");
        let instruction_array: Vec<u16> = instruction_string
            .split("\n")
            .map(|x| parse_instruction(x).expect("Invalid instruction"))
            .collect();
        let mut memory_array: [u16; 4096] = [0; 4096];
        for (i, instruction) in instruction_array.iter().enumerate() {
            memory_array[i] = *instruction;
        }
        memory_array.map(|x| match x {
            0 => (),
            _ => println!("{:#06X}", x),
        });

        return Processor {
            clock_cycle: 0,
            registers: [0; 16],
            memory: memory_array,
            instruction_register: 0,
            control_signals: ControlSignals { terminate: false },
            state_machine: StateMachine::new(),
        };
    }

    // returns false if the processor should terminate
    pub fn run(&mut self) -> bool {
        self.state_machine.next_state();
        self.control_signals = self.state_machine.get_control_signals();
        if self.control_signals.terminate {
            return false;
        }
        // do things with the control signals
        return true;
    }

    pub fn fetch_instruction() {
        todo!()
    }

    pub fn decode_instruction(instruction: u32) -> InstructionToken {
        let opcode: Opcode =
            Opcode::from_u8(u8::try_from((instruction & 0xF000) >> 12).expect("Invalid byte 1"));
        let byte_2: u8 = u8::try_from((instruction & 0x0F00) >> 8).expect("Invalid byte 2");
        let byte_3: u8 = u8::try_from((instruction & 0x00F0) >> 4).expect("Invalid byte 3");
        let byte_4: u8 = u8::try_from(instruction & 0x000F).expect("Invalid byte 4");
        let instruction_type: InstructionType = InstructionType::from_opcode(&opcode);
        return InstructionToken {
            opcode,
            byte_2,
            byte_3,
            byte_4,
            instruction_type,
        };
    }
}
