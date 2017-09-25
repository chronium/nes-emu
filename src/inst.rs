use cpu::NMOS6502;

#[derive(Debug, PartialEq)]
pub enum Value {
    Implied,
    Immediate(u8),
}

macro_rules! immediate {
    ($instr:ident, $cpu:ident) => (Instruction(Opcode::$instr, Value::Immediate($cpu.read8_pc())))
}

#[derive(Debug)]
pub enum Opcode {
    LDA,    // A9
    Unknown(u8),
}

#[derive(Debug)]
pub struct Instruction(pub Opcode, pub Value);

impl<'a> From<&'a mut NMOS6502> for Instruction {
    fn from(cpu: &'a mut NMOS6502) -> Self {
        match cpu.read8_pc() {
            0xA9 => immediate!(LDA, cpu),
            op => Instruction(Opcode::Unknown(op), Value::Implied),
        }
    }
}