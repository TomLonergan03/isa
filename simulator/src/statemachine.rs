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
        todo!()
    }

    pub fn next_state(&mut self) {
        todo!()
    }
}
