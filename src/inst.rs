use cpu::NMOS6502;

use std::cell::RefCell;
use std::rc::Rc;

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

impl Instruction {
    pub fn get(cpu: &mut NMOS6502) -> (u16, Self) {
        match {cpu.read8_pc()} {
            0xA9 => (2, immediate!(LDA, cpu)),
            op => (0, Instruction(Opcode::Unknown(op), Value::Implied)),
        }
    }
}