use crate::types::{ControlSignals, State};
use log::debug;

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
            State::Terminate => ControlSignals { terminate: true },
            _ => ControlSignals { terminate: false },
        }
    }

    pub fn next_state(&mut self, instruction_token: &crate::types::InstructionToken) {
        match self.state {
            State::InstructionFetch => {
                debug!("Entering state: Decode");
                self.state = State::Decode;
            }
            State::Decode => match instruction_token.opcode {
                _ => {
                    debug!("Entering state: Terminate");
                    self.state = State::Terminate;
                }
            },
            State::Terminate => {
                println!("Program terminated, memory and registers dumped");
            }
        }
    }
}
