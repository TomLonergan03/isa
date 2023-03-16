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
    Invalid,
}

impl Opcode {
    pub fn from_u8(value: u8) -> Opcode {
        return match value {
            0x00 => Opcode::Add,
            0x01 => Opcode::Subtract,
            0x02 => Opcode::Multiply,
            0x03 => Opcode::Divide,
            0x04 => Opcode::And,
            0x05 => Opcode::Or,
            0x06 => Opcode::SetIfLessThan,
            0x07 => Opcode::SetIfEqual,
            0x08 => Opcode::ShiftLeft,
            0x09 => Opcode::ShiftRightLogical,
            0x0A => Opcode::ShiftRightArithmetic,
            0x0B => Opcode::SetLower,
            0x0C => Opcode::SetUpper,
            0x0D => Opcode::LoadWord,
            0x0E => Opcode::SaveWord,
            0x0F => Opcode::Special,
            _ => Opcode::Invalid,
        };
    }
}

pub enum InstructionType {
    Register,
    Set,
    Memory,
    Special,
    Invalid,
}

impl InstructionType {
    pub fn from_opcode(opcode: &Opcode) -> InstructionType {
        return match opcode {
            Opcode::Add => InstructionType::Register,
            Opcode::Subtract => InstructionType::Register,
            Opcode::Multiply => InstructionType::Register,
            Opcode::Divide => InstructionType::Register,
            Opcode::And => InstructionType::Register,
            Opcode::Or => InstructionType::Register,
            Opcode::SetIfLessThan => InstructionType::Register,
            Opcode::SetIfEqual => InstructionType::Register,
            Opcode::ShiftLeft => InstructionType::Register,
            Opcode::ShiftRightLogical => InstructionType::Register,
            Opcode::ShiftRightArithmetic => InstructionType::Register,
            Opcode::SetLower => InstructionType::Set,
            Opcode::SetUpper => InstructionType::Set,
            Opcode::LoadWord => InstructionType::Memory,
            Opcode::SaveWord => InstructionType::Memory,
            Opcode::Special => InstructionType::Special,
            Opcode::Invalid => InstructionType::Invalid,
        };
    }
}

pub enum State {
    InstructionFetch,
    Decode,
    Terminate,
}

pub struct InstructionToken {
    pub opcode: Opcode,
    pub nibble_2: u8,
    pub nibble_3: u8,
    pub nibble_4: u8,
    pub instruction_type: InstructionType,
}

pub enum AddressSource {
    ALU,
    ProgramCounter,
}

pub enum RegisterSource {
    Instruction,
    Memory,
    Alu,
    AluZero,
    AluNegative,
}

pub enum AluOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    ShiftLeft,
    ShiftRightLogical,
    ShiftRightArithmetic,
}

pub struct PipelineRegisters {
    pub memory_data: u16,
    pub register_read_a: u16,
    pub register_read_b: u16,
    pub alu_output: u32,
}
pub struct ControlSignals {
    pub decode: bool,
    pub terminate: bool,
    pub address_source: AddressSource,
    pub memory_read: bool,
    pub memory_write: bool,
    pub instruction_register_write: bool,
    pub register_write: bool,
    pub upper_register_write: bool,
    pub long_register_write: bool,
    pub read_pc: bool,
    pub write_pc: bool,
    pub write_register_source: RegisterSource,
    pub alu_operation: AluOperation,
}
