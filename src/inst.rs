use cpu::NMOS6502;

#[derive(Debug, PartialEq)]
pub enum Value {
    Implied,
    PreIdxInd(u8),
    Immediate(u8),
    Absolute(u16),
    Relative(i8),
    ZeroPage(u8),
    ZeroPageX(u8),
    AbsoluteX(u16),
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

macro_rules! izx {
    ($instr:ident, $cpu:ident) => {{ (2, (Instruction(Opcode::$instr, Value::PreIdxInd($cpu.read8_pc())))) }}
}

macro_rules! zpx {
    ($instr:ident, $cpu:ident) => {{ (2, (Instruction(Opcode::$instr, Value::ZeroPageX($cpu.read8_pc())))) }}
}

macro_rules! abx {
    ($instr:ident, $cpu:ident) => {{ (3, (Instruction(Opcode::$instr, Value::AbsoluteX($cpu.read16_pc())))) }}
}

#[derive(Debug)]
pub enum Opcode {
    PHP,    // 08
    ORA,    // 01 09
    ASL,    // 0A
    BPL,    // 10
    CLC,    // 18
    JSR,    // 20
    BIT,    // 24 2C
    PLP,    // 28
    AND,    // 21 29
    ROL,    // 2A
    BMI,    // 30
    SEC,    // 38
    RTI,    // 40
    PHA,    // 48
    EOR,    // 41 49
    LSR,    // 4A
    JMP,    // 4C
    BVC,    // 50
    RTS,    // 60
    PLA,    // 68
    ADC,    // 61 65 69
    ROR,    // 6A
    BVS,    // 70
    SEI,    // 78
    STX,    // 86 8E
    STA,    // 81 85 8D 95 9D
    DEY,    // 88
    TXA,    // 8A
    BCC,    // 90
    TYA,    // 98
    TXS,    // 9A
    LDY,    // A0
    LDX,    // A2 AE
    LDA,    // A1 A5 A9 AD
    TAY,    // A8
    TAX,    // AA
    BCS,    // B0
    CLV,    // B8
    TSX,    // BA
    CPY,    // C0
    INY,    // C8
    CMP,    // C1 C9
    DEX,    // CA
    BNE,    // D0
    CLD,    // D8
    CPX,    // E0
    INX,    // E8
    SBC,    // E1 E9
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
            0x01 => izx!(ORA, cpu),
            0x08 => imp!(PHP, cpu),
            0x09 => imm!(ORA, cpu),
            0x0A => imp!(ASL, cpu),
            0x10 => rel!(BPL, cpu),
            0x18 => imp!(CLC, cpu),
            0x30 => rel!(BMI, cpu),
            0x38 => imp!(SEC, cpu),
            0x20 => abs!(JSR, cpu),
            0x21 => izx!(AND, cpu),
            0x24 => zpg!(BIT, cpu),
            0x28 => imp!(PLP, cpu),
            0x29 => imm!(AND, cpu),
            0x2A => imp!(ROL, cpu),
            0x2C => abs!(BIT, cpu),
            0x40 => imp!(RTI, cpu),
            0x41 => izx!(EOR, cpu),
            0x48 => imp!(PHA, cpu),
            0x49 => imm!(EOR, cpu),
            0x4a => imp!(LSR, cpu),
            0x4C => abs!(JMP, cpu),
            0x50 => rel!(BVC, cpu),
            0x60 => imp!(RTS, cpu),
            0x61 => izx!(ADC, cpu),
            0x65 => zpg!(ADC, cpu),
            0x68 => imp!(PLA, cpu),
            0x69 => imm!(ADC, cpu),
            0x6A => imp!(ROR, cpu),
            0x70 => rel!(BVS, cpu),
            0x78 => imp!(SEI, cpu),
            0x81 => izx!(STA, cpu),
            0x85 => zpg!(STA, cpu),
            0x86 => zpg!(STX, cpu),
            0x88 => imp!(DEY, cpu),
            0x8A => imp!(TXA, cpu),
            0x8D => abs!(STA, cpu),
            0x8E => abs!(STX, cpu),
            0x90 => rel!(BCC, cpu),
            0x95 => zpx!(STA, cpu),
            0x98 => imp!(TYA, cpu),
            0x9A => imp!(TXS, cpu),
            0x9D => abx!(STA, cpu),
            0xA0 => imm!(LDY, cpu),
            0xA1 => izx!(LDA, cpu),
            0xA2 => imm!(LDX, cpu),
            0xA5 => zpg!(LDA, cpu),
            0xA8 => imp!(TAY, cpu),
            0xA9 => imm!(LDA, cpu),
            0xAA => imp!(TAX, cpu),
            0xAD => abs!(LDA, cpu),
            0xAE => abs!(LDX, cpu),
            0xB0 => rel!(BCS, cpu),
            0xB8 => imp!(CLV, cpu),
            0xBA => imp!(TSX, cpu),
            0xC0 => imm!(CPY, cpu),
            0xC1 => izx!(CMP, cpu),
            0xC8 => imp!(INY, cpu),
            0xC9 => imm!(CMP, cpu),
            0xCA => imp!(DEX, cpu),
            0xD0 => rel!(BNE, cpu),
            0xD8 => imp!(CLD, cpu),
            0xE0 => imm!(CPX, cpu),
            0xE1 => izx!(SBC, cpu),
            0xE8 => imp!(INX, cpu),
            0xE9 => imm!(SBC, cpu),
            0xEA => imp!(NOP, cpu),
            0xF0 => rel!(BEQ, cpu),
            0xF8 => imp!(SED, cpu),
            op => (0, Instruction(Opcode::Unknown(op), Value::Implied)),
        }
    }
}