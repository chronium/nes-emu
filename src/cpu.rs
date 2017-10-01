use ppuregs::PPUCTL;
use inst::{Instruction, Value, Opcode};
use mem::Memory;
use ppu::PPU;

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
        const FLAG_N = 0b10000000;
    }
}

#[derive(Debug, Clone)]
pub struct NMOS6502 {
    mem: Box<Memory>,
    ppu: Rc<RefCell<PPU>>,

    a: u8,

    x: u8,
    y: u8,

    sp: u8,

    pc: u16,

    p_flags: PFlag,
}

impl NMOS6502 {
    pub fn new(mem: Box<Memory>, ppu: Rc<RefCell<PPU>>) -> Self {
        NMOS6502 {
            mem: mem,
            ppu: ppu,
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

    pub fn step(&mut self) -> Result<u8, String> {
        print!("0x{:4X}: ", self.pc);
        let (adv, inst) = Instruction::get(&mut self.clone());
        self.pc += adv;
        match inst {
            Instruction(Opcode::LDA, Value::Immediate(val)) => {
                println!("LDA #{:X}", val);

                self.a = val;

                if val == 0 {
                    self.p_flags.insert(PFlag::FLAG_Z);
                }

                if val & 0x80 == 0x80 {
                    self.p_flags.insert(PFlag::FLAG_N);
                }

                Ok(0u8)
            }
            Instruction(Opcode::LDA, Value::Absolute(addr)) => {
                println!("LDA @{:04X}", addr);
                let val = self.read8(addr);
                self.a = val;

                if val == 0 {
                    self.p_flags.insert(PFlag::FLAG_Z);
                }

                if val & 0x80 == 0x80 {
                    self.p_flags.insert(PFlag::FLAG_N);
                }

                Ok(0u8)
            }
            Instruction(Opcode::STA, Value::Absolute(addr)) => {
                println!("STA @{:04X}", addr);

                let a = self.a;
                self.write8(addr, a);

                Ok(0u8)
            }
            Instruction(Opcode::BPL, Value::Relative(offs)) => {
                println!("BPL +{:02X}", offs);

                if !self.p_flags.contains(PFlag::FLAG_N) {
                    self.pc = self.pc & 0xFF00 | (self.pc & 0xFF + offs as u16);
                }

                Ok(0u8)
            }
            Instruction(Opcode::BMI, Value::Relative(offs)) => {
                println!("BPL +{:02X}", offs);

                if self.p_flags.contains(PFlag::FLAG_N) {
                    self.pc = self.pc & 0xFF00 | (self.pc & 0xFF + offs as u16);
                }

                Ok(0u8)
            }
            instr => Err(format!("{:?}", instr)),
        }
    }

    pub fn read8(&mut self, addr: u16) -> u8 {
        match addr {
            0x2002 => self.ppu.borrow().ppustatus,
            _ => panic!(),
        }
    }

    pub fn read8_pc(&mut self) -> u8 {
        let ret = self.mem.read8(self.pc);
        self.pc += 1;
        ret
    }

    pub fn read16_pc(&mut self) -> u16 {
        let ret = (self.mem.read8(self.pc) as u16) | ((self.mem.read8(self.pc + 1) as u16) << 8);
        self.pc += 2;
        ret
    }

    fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            0x2000 => { 
                println!("PPUCTRL {:X}", val);
                self.ppu.borrow_mut().ppuctl = PPUCTL::from(val);
            }
            0x2001 => println!("PPUMASK {:X}", val),
            _ => unimplemented!(),
        }
    }
}