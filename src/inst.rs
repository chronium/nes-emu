use cpu::NMOS6502;

#[derive(Debug, PartialEq)]
pub enum Value {
    Implied,
    Immediate(u8),
    Absolute(u16),
    Relative(i8),
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
    ($instr:ident, $cpu:ident) => {{ (2, (Instruction(Opcode::$instr, Value::Relative($cpu.read8_pc() as i8)))) }}
}

macro_rules! zpg {
    ($instr:ident, $cpu:ident) => {{ (2, (Instruction(Opcode::$instr, Value::ZeroPage($cpu.read8_pc())))) }}
}

#[derive(Debug)]
pub enum Opcode {
    PHP,    // 08
    ORA,    // 09
    BPL,    // 10
    CLC,    // 18
    JSR,    // 20
    BIT,    // 24
    PLP,    // 28
    AND,    // 29
    BMI,    // 30
    SEC,    // 38
    PHA,    // 48
    JMP,    // 4C
    BVC,    // 50
    RTS,    // 60
    PLA,    // 68
    BVS,    // 70
    SEI,    // 78
    STX,    // 86
    STA,    // 85 8D
    BCC,    // 90
    TXS,    // 9A
    LDX,    // A2
    LDA,    // A9 AD
    BCS,    // B0
    CLV,    // B8
    CMP,    // C9
    BNE,    // D0
    CLD,    // D8
    NOP,    // EA
    BEQ,    // F0
    SED,    // F8
    Unknown(u8),
}

#[derive(Debug)]
pub struct Instruction(pub Opcode, pub Value);

impl Instruction {
    pub fn get(cpu: &mut NMOS6502) -> (u16, Self) {
        match {cpu.read8_pc()} {
            0x08 => imp!(PHP, cpu),
            0x09 => imm!(ORA, cpu),
            0x10 => rel!(BPL, cpu),
            0x18 => imp!(CLC, cpu),
            0x30 => rel!(BMI, cpu),
            0x38 => imp!(SEC, cpu),
            0x20 => abs!(JSR, cpu),
            0x24 => zpg!(BIT, cpu),
            0x28 => imp!(PLP, cpu),
            0x29 => imm!(AND, cpu),
            0x48 => imp!(PHA, cpu),
            0x4C => abs!(JMP, cpu),
            0x50 => rel!(BVC, cpu),
            0x60 => imp!(RTS, cpu),
            0x68 => imp!(PLA, cpu),
            0x70 => rel!(BVS, cpu),
            0x78 => imp!(SEI, cpu),
            0x85 => zpg!(STA, cpu),
            0x86 => zpg!(STX, cpu),
            0x8D => abs!(STA, cpu),
            0x90 => rel!(BCC, cpu),
            0x9A => imp!(TXS, cpu),
            0xA2 => imm!(LDX, cpu),
            0xA9 => imm!(LDA, cpu),
            0xAD => abs!(LDA, cpu),
            0xB0 => rel!(BCS, cpu),
            0xB8 => imp!(CLV, cpu),
            0xC9 => imm!(CMP, cpu),
            0xD0 => rel!(BNE, cpu),
            0xD8 => imp!(CLD, cpu),
            0xEA => imp!(NOP, cpu),
            0xF0 => rel!(BEQ, cpu),
            0xF8 => imp!(SED, cpu),
            op => (0, Instruction(Opcode::Unknown(op), Value::Implied)),
        }
    }
}