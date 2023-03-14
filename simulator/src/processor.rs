use crate::instructions::Instructions;
use crate::statemachine::StateMachine;
use crate::types::{ControlSignals, InstructionToken, InstructionType, Opcode};

pub struct Processor {
    clock_cycle: u64,
    registers: [u16; 16],
    memory: [u8; 8192],
    instruction_register: u16,
    control_signals: ControlSignals,
    state_machine: StateMachine,
    instructions: Instructions,
}

impl Processor {
    pub fn new(instruction_path: String) -> Processor {
        return Processor {
            clock_cycle: 0,
            registers: [0; 16],
            memory: [0; 8192],
            instruction_register: 0,
            control_signals: ControlSignals { terminate: false },
            state_machine: StateMachine::new(),
            instructions: Instructions::new(instruction_path),
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
