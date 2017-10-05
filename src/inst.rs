use cpu::NMOS6502;

#[derive(Debug, PartialEq)]
pub enum Value {
    Implied,
    Immediate(u8),
    Absolute(u16),
    Relative(u8),
    ZeroPage(u8),
}

macro_rules! imp {
    ($instr:ident, $cpu:ident) => {{ (1, (Instruction(Opcode::$instr, Value::Implied))) }}
}

macro_rules! imm {
    ($instr:ident, $cpu:ident) => {{ (2, (Instruction(Opcode::$instr, Value::Immediate($cpu.read8_pc())))) }}
}

macro_rules! abs {
    ($instr:ident, $cpu:ident) => {{ (3, (Instruction(Opcode::$instr, Value::Absolute($cpu.read16_pc())))) }}
}

macro_rules! rel {
    ($instr:ident, $cpu:ident) => {{ (2, (Instruction(Opcode::$instr, Value::Relative($cpu.read8_pc())))) }}
}

macro_rules! zpg {
    ($instr:ident, $cpu:ident) => {{ (2, (Instruction(Opcode::$instr, Value::ZeroPage($cpu.read8_pc())))) }}
}

#[derive(Debug)]
pub enum Opcode {
    BPL,    // 10
    CLC,    // 18
    JSR,    // 20
    BIT,    // 24
    BMI,    // 30
    SEC,    // 38
    JMP,    // 4C
    SEI,    // 78
    STX,    // 86
    STA,    // 85 8D
    BCC,    // 90
    LDX,    // A2
    LDA,    // A9 AD
    BCS,    // B0
    BNE,    // D0
    CLD,    // D8
    NOP,    // EA
    BEQ,    // F0
    Unknown(u8),
}

#[derive(Debug)]
pub struct Instruction(pub Opcode, pub Value);

impl Instruction {
    pub fn get(cpu: &mut NMOS6502) -> (u16, Self) {
        match {cpu.read8_pc()} {
            0x10 => rel!(BPL, cpu),
            0x18 => imp!(CLC, cpu),
            0x30 => rel!(BMI, cpu),
            0x38 => imp!(SEC, cpu),
            0x20 => abs!(JSR, cpu),
            0x24 => zpg!(BIT, cpu),
            0x4C => abs!(JMP, cpu),
            0x78 => imp!(SEI, cpu),
            0x85 => zpg!(STA, cpu),
            0x86 => zpg!(STX, cpu),
            0x8D => abs!(STA, cpu),
            0x90 => rel!(BCC, cpu),
            0xA2 => imm!(LDX, cpu),
            0xA9 => imm!(LDA, cpu),
            0xAD => abs!(LDA, cpu),
            0xB0 => rel!(BCS, cpu),
            0xD0 => rel!(BNE, cpu),
            0xD8 => imp!(CLD, cpu),
            0xEA => imp!(NOP, cpu),
            0xF0 => rel!(BEQ, cpu),
            op => (0, Instruction(Opcode::Unknown(op), Value::Implied)),
        }
    }
}