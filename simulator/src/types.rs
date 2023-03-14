pub enum Opcode {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    SetIfLessThan,
    SetIfEqual,
    ShiftLeft,
    ShiftRightLogical,
    ShiftRightArithmetic,
    SetLower,
    SetUpper,
    LoadWord,
    SaveWord,
    Special,
}

pub enum InstructionType {
    Register,
    Set,
    Memory,
    Special,
}

pub enum State {
    InstructionFetch,
    Decode,
    Terminate,
}

pub struct InstructionToken {
    pub opcode: Opcode,
    pub byte_2: u8,
    pub byte_3: u8,
    pub byte_4: u8,
    pub instruction_type: InstructionType,
}

pub struct ControlSignals {
    pub terminate: bool,
}
