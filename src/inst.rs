use cpu::NMOS6502;

#[derive(Debug, PartialEq)]
pub enum Value {
    Implied,
    Immediate(u8),
    Absolute(u16),
    Relative(u8),
    ZeroPage(u8),
}

macro_rules! imm {
    ($instr:ident, $cpu:ident) => (Instruction(Opcode::$instr, Value::Immediate($cpu.read8_pc())))
}

macro_rules! abs {
    ($instr:ident, $cpu:ident) => (Instruction(Opcode::$instr, Value::Absolute($cpu.read16_pc())))
}

macro_rules! rel {
    ($instr:ident, $cpu:ident) => (Instruction(Opcode::$instr, Value::Relative($cpu.read8_pc())))
}

macro_rules! zpg {
    ($instr:ident, $cpu:ident) => (Instruction(Opcode::$instr, Value::ZeroPage($cpu.read8_pc())))
}

#[derive(Debug)]
pub enum Opcode {
    BPL,    // 10
    JSR,    // 20
    BMI,    // 30
    JMP,    // 4C
    STX,    // 86
    STA,    // 8D
    LDX,    // A2
    LDA,    // A9 AD
    Unknown(u8),
}

#[derive(Debug)]
pub struct Instruction(pub Opcode, pub Value);

impl Instruction {
    pub fn get(cpu: &mut NMOS6502) -> (u16, Self) {
        match {cpu.read8_pc()} {
            0x10 => (2, rel!(BPL, cpu)),
            0x30 => (2, rel!(BMI, cpu)),
            0x20 => (3, abs!(JSR, cpu)),
            0x4C => (3, abs!(JMP, cpu)),
            0x86 => (2, zpg!(STX, cpu)),
            0x8D => (3, abs!(STA, cpu)),
            0xA2 => (2, imm!(LDX, cpu)),
            0xA9 => (2, imm!(LDA, cpu)),
            0xAD => (3, abs!(LDA, cpu)),
            op => (0, Instruction(Opcode::Unknown(op), Value::Implied)),
        }
    }
}