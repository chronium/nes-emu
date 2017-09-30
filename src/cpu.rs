use inst::{Instruction, Value};
use mem::Memory;

use std::cell::RefCell;
use std::rc::Rc;

bitflags! {
    #[derive(Default)]
    pub struct PFlag: u8 {
        const FLAG_C = 0b00000001;
        const FLAG_Z = 0b00000010;
        const FLAG_I = 0b00000100;
        const FLAG_D = 0b00001000;
        const FLAG_B = 0b00010000;
        const FLAG_V = 0b01000000;
        const FLAG_S = 0b10000000;
    }
}

#[derive(Debug, Clone)]
pub struct NMOS6502 {
    mem: Box<Memory>,

    a: u8,

    x: u8,
    y: u8,

    sp: u8,

    pc: u16,

    p_flags: PFlag,
}

impl NMOS6502 {
    pub fn new(mem: Box<Memory>) -> Self {
        NMOS6502 {
            mem: mem,
            a: 0u8,
            x: 0u8,
            y: 0u8,
            sp: 0u8,
            pc: 0u16,
            p_flags: PFlag::empty(),
        }
    }

    pub fn reset(&mut self) {
        self.pc = (self.mem.read8(0xFFFD) as u16) << 8 | (self.mem.read8(0xFFFC) as u16);
    }

    pub fn step(&mut self) {
        print!("0x{:4X}: ", self.pc);
        let (adv, inst) = Instruction::get(&mut self.clone());
        self.pc += adv;
        match inst {
            Instruction(LDA, Value::Immediate(val)) => {
                println!("LDA $#{:X}", val);

                self.a = val;

                if val == 0 {
                    self.p_flags.insert(PFlag::FLAG_Z);
                }

                if val & 0x80 == 0x80 {
                    self.p_flags.insert(PFlag::FLAG_C);
                }
            }
            instr => println!("{:?}", instr),
        }
    }

    pub fn read8_pc(&mut self) -> u8 {
        let ret = self.mem.read8(self.pc);
        self.pc += 1;
        ret
    }
}