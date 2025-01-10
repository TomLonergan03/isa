/// Opcode representations
#[derive(Clone, PartialEq, Debug)]
pub enum Opcode {
    Add,
    AddImmediate,
    Subtract,
    SubtractImmediate,
    And,
    Or,
    Xor,
    Invert,
    SetIfLess,
    SetIfEqual,
    SetIfNotEqual,
    SetIfGreater,
    ShiftLeft,
    ShiftRightLogical,
    ShiftRightArithmetic,
    SetLower,
    SetUpper,
    LoadWord,
    SaveWord,
    BranchIfEqual,
    BranchIfNotEqual,
    BranchIfLess,
    BranchIfGreater,
    Special,
    Invalid,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    Register {
        opcode: Opcode,
        destination: u8,
        source1: u8,
        source2: u8,
    },
    Immediate {
        opcode: Opcode,
        destination: u8,
        source1: u8,
        immediate: u16,
    },
    Memory {
        opcode: Opcode,
        destination: u8,
        source1: u8,
        address: u16,
    },
    Special {
        opcode: Opcode,
        subinstruction: u16,
    },
    Invalid,
}
