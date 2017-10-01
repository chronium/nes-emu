use cpu::NMOS6502;

#[derive(Debug, PartialEq)]
pub enum Value {
    Implied,
    Immediate(u8),
    Absolute(u16),
    Relative(u8),
}

macro_rules! immediate {
    ($instr:ident, $cpu:ident) => (Instruction(Opcode::$instr, Value::Immediate($cpu.read8_pc())))
}

macro_rules! absolute {
    ($instr:ident, $cpu:ident) => (Instruction(Opcode::$instr, Value::Absolute($cpu.read16_pc())))
}

macro_rules! relative {
    ($instr:ident, $cpu:ident) => (Instruction(Opcode::$instr, Value::Relative($cpu.read8_pc())))
}

#[derive(Debug)]
pub enum Opcode {
    BPL,    // 10
    BMI,    // 30
    LDA,    // A9 AD
    STA,    // 8D
    Unknown(u8),
}

#[derive(Debug)]
pub struct Instruction(pub Opcode, pub Value);

impl Instruction {
    pub fn get(cpu: &mut NMOS6502) -> (u16, Self) {
        match {cpu.read8_pc()} {
            0x10 => (2, relative!(BPL, cpu)),
            0x30 => (2, relative!(BMI, cpu)),
            0x8D => (3, absolute!(STA, cpu)),
            0xA9 => (2, immediate!(LDA, cpu)),
            0xAD => (3, absolute!(LDA, cpu)),
            op => (0, Instruction(Opcode::Unknown(op), Value::Implied)),
        }
    }
}