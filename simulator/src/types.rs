use log::error;

#[derive(Clone, PartialEq, Debug)]
pub enum Opcode {
    Add,
    Subtract,
    And,
    Or,
    SetIfLess,
    SetIfEqual,
    ShiftLeft,
    ShiftRightLogical,
    ShiftRightArithmetic,
    SetLower,
    SetUpper,
    LoadWord,
    SaveWord,
    Special,
    SetPcIf,
    Invalid,
}

impl Opcode {
    pub fn from_u8(value: u8) -> Opcode {
        return match value {
            0x00 => Opcode::Add,
            0x01 => Opcode::Subtract,
            0x02 => Opcode::And,
            0x03 => Opcode::Or,
            0x04 => Opcode::SetIfLess,
            0x05 => Opcode::SetIfEqual,
            0x06 => Opcode::ShiftLeft,
            0x07 => Opcode::ShiftRightLogical,
            0x08 => Opcode::ShiftRightArithmetic,
            0x09 => Opcode::SetLower,
            0x0A => Opcode::SetUpper,
            0x0B => Opcode::LoadWord,
            0x0C => Opcode::SaveWord,
            0x0D => Opcode::SetPcIf,
            0x0F => Opcode::Special,
            _ => Opcode::Invalid,
        };
    }
}
#[derive(Clone)]
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
            Opcode::And => InstructionType::Register,
            Opcode::Or => InstructionType::Register,
            Opcode::SetIfLess => InstructionType::Register,
            Opcode::SetIfEqual => InstructionType::Register,
            Opcode::ShiftLeft => InstructionType::Register,
            Opcode::ShiftRightLogical => InstructionType::Register,
            Opcode::ShiftRightArithmetic => InstructionType::Register,
            Opcode::SetLower => InstructionType::Set,
            Opcode::SetUpper => InstructionType::Set,
            Opcode::LoadWord => InstructionType::Memory,
            Opcode::SaveWord => InstructionType::Memory,
            Opcode::Special => InstructionType::Special,
            Opcode::SetPcIf => InstructionType::Special,
            Opcode::Invalid => InstructionType::Invalid,
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    PcRead,
    InstructionFetch,
    Decode,
    SetLower,
    SetUpper,
    ArithmeticOperation,
    ArithmeticWriteBack,
    SetIf,
    SetIfLess,
    SetIfEqual,
    Memory,
    MemoryRead,
    MemoryReadRegisterWriteback,
    MemoryWrite,
    SetPcTest,
    SetPcWriteback,
    Special,
    Terminate,
}

#[derive(Clone)]
pub struct InstructionToken {
    pub opcode: Opcode,
    pub nibble_2: u8,
    pub nibble_3: u8,
    pub nibble_4: u8,
    pub instruction_type: InstructionType,
}

pub enum AddressSource {
    Alu,
    ProgramCounter,
}

pub enum RegisterWriteSource {
    InstructionByte2,
    Memory,
    Alu,
    AluZero,
    AluNegative,
    InstructionNibble2,
}
#[derive(Debug, Clone, PartialEq)]
pub enum AluOperation {
    Add,
    Subtract,
    And,
    Or,
    ShiftLeft,
    ShiftRightLogical,
    ShiftRightArithmetic,
    Inactive,
}

impl AluOperation {
    pub fn from_opcode(opcode: &Opcode) -> AluOperation {
        return match opcode {
            Opcode::Add => AluOperation::Add,
            Opcode::Subtract => AluOperation::Subtract,
            Opcode::And => AluOperation::And,
            Opcode::Or => AluOperation::Or,
            Opcode::ShiftLeft => AluOperation::ShiftLeft,
            Opcode::ShiftRightLogical => AluOperation::ShiftRightLogical,
            Opcode::ShiftRightArithmetic => AluOperation::ShiftRightArithmetic,
            Opcode::SetIfEqual => AluOperation::Subtract,
            Opcode::SetIfLess => AluOperation::Subtract,
            _ => {
                error!("Invalid opcode for ALU operation");
                panic!("Invalid opcode for ALU operation")
            }
        };
    }
}

pub enum AluSource {
    Register,
    Constant1,
    MemoryOffset,
}

pub enum RegisterWriteTarget {
    Nibble2,
    Nibble3,
}

#[derive(PartialEq)]
pub enum RunState {
    Stop,
    Continue,
}

pub struct AluOutput {
    pub result: u32,
    pub zero: bool,
    pub negative: bool,
}

pub struct PipelineRegisters {
    pub memory_data: u16,
    pub register_read_a: u16,
    pub register_read_b: u16,
    pub alu_output: u32,
    pub alu_negative: bool,
    pub alu_zero: bool,
}

pub struct ControlSignals {
    pub terminate: bool,
    pub decode: bool,
    pub address_source: AddressSource,
    pub memory_read: bool,
    pub memory_write: bool,
    pub instruction_register_write: bool,
    pub register_write: bool,
    pub register_write_source: RegisterWriteSource,
    pub write_upper: bool,
    pub write_long: bool,
    pub read_pc: bool,
    pub write_pc: bool,
    pub alu_operation: AluOperation,
    pub alu_source: AluSource,
    pub process_special: bool,
    pub write_register_target: RegisterWriteTarget,
}
