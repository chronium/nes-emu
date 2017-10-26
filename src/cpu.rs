use ppuregs::PPUCTL;
use inst::{Instruction, Value, Opcode};
use mem::Memory;
use ppu::PPU;

use std::cell::RefCell;
use std::rc::Rc;

use std::fmt;

bitflags! {
    #[derive(Default)]
    pub struct PFlag: u8 {
        const FLAG_C = 0b00000001;
        const FLAG_Z = 0b00000010;
        const FLAG_I = 0b00000100;
        const FLAG_D = 0b00001000;
        const FLAG_B = 0b00010000;
        const FLAG_X = 0b00100000;
        const FLAG_V = 0b01000000;
        const FLAG_N = 0b10000000;
    }
}

#[derive(Debug, Clone)]
pub struct NMOS6502 {
    mem: Rc<RefCell<Memory>>,
    ppu: Rc<RefCell<PPU>>,

    a: u8,

    x: u8,
    y: u8,

    sp: u8,

    pc: u16,

    p_flags: PFlag,
}

impl NMOS6502 {
    pub fn new(mem: Rc<RefCell<Memory>>, ppu: Rc<RefCell<PPU>>) -> Self {
        let mut p = PFlag::empty();
        p.insert(PFlag::FLAG_I);
        p.insert(PFlag::FLAG_X);
        NMOS6502 {
            mem: mem,
            ppu: ppu,
            a: 0u8,
            x: 0u8,
            y: 0u8,
            sp: 0xFFu8,
            pc: 0u16,
            p_flags: p,
        }
    }

    pub fn reset(&mut self) {
        self.pc = (self.mem.borrow().read8(0xFFFD) as u16) << 8 | (self.mem.borrow().read8(0xFFFC) as u16);
    }

    pub fn step(&mut self) -> Result<u8, String> {
        print!("0x{:04X}: ", self.pc);
        let (adv, inst) = Instruction::get(&mut self.clone());
        self.pc += adv;
        match inst {
            Instruction(Opcode::LDA, Value::Immediate(val)) => {
                println!("LDA #${:02X}", val);

                self.a = val;

                self.set_nz(val);

                Ok(0u8)
            }
            Instruction(Opcode::LDA, Value::Absolute(addr)) => {
                let val = self.read8(addr);
                println!("LDA @{:04X} = {:02X}", addr, val);
                self.a = val;

                self.set_nz(val);

                Ok(0u8)
            }
            Instruction(Opcode::STA, Value::Absolute(addr)) => {
                println!("STA @{:04X} = {:02X}", addr, self.read8(addr));

                let a = self.a;
                self.write8(addr, a);

                Ok(0u8)
            }
            Instruction(Opcode::BPL, Value::Relative(offs)) => {
                let pc = ((self.pc as i32 + offs as i32) & 0xFFFF) as u16;
                println!("BPL ${:04X}", pc);

                if !self.p_flags.contains(PFlag::FLAG_N) {
                    self.pc = pc;
                }

                Ok(0u8)
            }
            Instruction(Opcode::BMI, Value::Relative(offs)) => {
                let pc = ((self.pc as i32 + offs as i32) & 0xFFFF) as u16;
                println!("BMI ${:04X}", pc);

                if self.p_flags.contains(PFlag::FLAG_N) {
                    self.pc = pc;
                }

                Ok(0u8)
            }
            Instruction(Opcode::JMP, Value::Absolute(addr)) => {
                println!("JMP ${:04X}", addr);

                self.pc = addr;

                Ok(0u8)
            }
            Instruction(Opcode::LDX, Value::Immediate(val)) => {
                println!("LDX #${:02X}", val);

                self.x = val;

                self.set_nz(val);

                Ok(0u8)
            }
            Instruction(Opcode::STX, Value::ZeroPage(zpg)) => {
                println!("STX ${:02X}", zpg);

                self.mem.borrow_mut().write8(zpg as u16, self.x);

                Ok(0u8)
            }
            Instruction(Opcode::JSR, Value::Absolute(addr)) => {
                println!("JSR ${:04X}", addr);

                let pc = self.pc;
                self.push16(pc - 1);
                self.pc = addr;

                Ok(0u8)
            }
            Instruction(Opcode::NOP, Value::Implied) => {
                println!("NOP");

                Ok(0u8)
            }
            Instruction(Opcode::SEC, Value::Implied) => {
                println!("SEC");

                self.set_carry(true);

                Ok(0u8)
            }
            Instruction(Opcode::BCS, Value::Relative(offs)) => {
                let pc = ((self.pc as i32 + offs as i32) & 0xFFFF) as u16;
                println!("BCS ${:04X}", pc);

                if self.p_flags.contains(PFlag::FLAG_C) {
                    self.pc = pc;
                }

                Ok(0u8)
            }
            Instruction(Opcode::CLC, Value::Implied) => {
                println!("CLC");

                self.set_carry(false);

                Ok(0u8)
            }
            Instruction(Opcode::BCC, Value::Relative(offs)) => {
                let pc = ((self.pc as i32 + offs as i32) & 0xFFFF) as u16;
                println!("BCC ${:04X}", pc);

                if !self.p_flags.contains(PFlag::FLAG_C) {
                    self.pc = pc;
                }

                Ok(0u8)
            }
            Instruction(Opcode::BEQ, Value::Relative(offs)) => {
                let pc = ((self.pc as i32 + offs as i32) & 0xFFFF) as u16;
                println!("BEQ ${:04X}", pc);

                if self.p_flags.contains(PFlag::FLAG_Z) {
                    self.pc = pc;
                }

                Ok(0u8)
            }
            Instruction(Opcode::BNE, Value::Relative(offs)) => {
                let pc = ((self.pc as i32 + offs as i32) & 0xFFFF) as u16;
                println!("BNE ${:04X}", pc);

                if !self.p_flags.contains(PFlag::FLAG_Z) {
                    self.pc = pc;
                }

                Ok(0u8)
            }
            Instruction(Opcode::SEI, Value::Implied) => {
                println!("SEI");

                self.p_flags.insert(PFlag::FLAG_I);

                Ok(0u8)
            }
            Instruction(Opcode::CLD, Value::Implied) => {
                println!("CLD");

                self.p_flags.remove(PFlag::FLAG_D);

                Ok(0u8)
            }
            Instruction(Opcode::STA, Value::ZeroPage(zpg)) => {
                println!("STA ${:02X} = {:02X}", zpg, self.read8(zpg as u16));

                self.mem.borrow_mut().write8(zpg as u16, self.a);

                Ok(0u8)
            }
            Instruction(Opcode::BIT, Value::ZeroPage(zpg)) => {
                let val = self.mem.borrow().read8(zpg as u16);
                println!("BIT ${:02X} = {:02X}", zpg, val);


                if val & 0x80 == 0x80 {
                    self.p_flags.insert(PFlag::FLAG_N);
                } else {
                    self.p_flags.remove(PFlag::FLAG_N);
                }

                if val & 0x40 == 0x40 {
                    self.p_flags.insert(PFlag::FLAG_V);
                } else {
                    self.p_flags.remove(PFlag::FLAG_V);
                }

                if val & self.a == 0 {
                    self.p_flags.insert(PFlag::FLAG_Z);
                } else {
                    self.p_flags.remove(PFlag::FLAG_Z);
                }

                Ok(0u8)
            }
            Instruction(Opcode::TXS, Value::Implied) => {
                println!("TXS");

                self.sp = self.x;

                Ok(0u8)
            }
            Instruction(Opcode::BVS, Value::Relative(offs)) => {
                let pc = ((self.pc as i32 + offs as i32) & 0xFFFF) as u16;
                println!("BVS ${:04X}", pc);

                if self.p_flags.contains(PFlag::FLAG_V) {
                    self.pc = pc;
                }

                Ok(0u8)
            }
            Instruction(Opcode::BVC, Value::Relative(offs)) => {
                let pc = ((self.pc as i32 + offs as i32) & 0xFFFF) as u16;
                println!("BVC ${:04X}", pc);

                if !self.p_flags.contains(PFlag::FLAG_V) {
                    self.pc = pc;
                }

                Ok(0u8)
            }
            Instruction(Opcode::RTS, Value::Implied) => {
                println!("RTS");

                self.pc = self.pop16() + 1;

                Ok(0u8)
            }
            Instruction(Opcode::PHP, Value::Implied) => {
                println!("PHP");

                let p = self.p_flags.bits;
                self.push8(p);

                Ok(0u8)
            }
            Instruction(Opcode::SED, Value::Implied) => {
                println!("SED");

                self.p_flags.insert(PFlag::FLAG_D);

                Ok(0u8)
            }
            Instruction(Opcode::PLA, Value::Implied) => {
                println!("PLA");
                let val = self.pop8();
                self.a = val;

                let a = self.a;
                self.set_nz(a);

                Ok(0u8)
            }
            Instruction(Opcode::AND, Value::Immediate(val)) => {
                println!("AND ${:02X}", val);
                let a = self.a & val;
                self.a = a;

                let a = self.a;
                self.set_nz(a);

                Ok(0u8)
            }
            Instruction(Opcode::CMP, Value::Immediate(val)) => {
                println!("CMP #${:02X}", val);

                let t = self.a;
                let temp = self.a.wrapping_sub(val);
                self.set_nz(temp);
                self.set_carry(t >= val);

                Ok(0u8)
            }
            Instruction(Opcode::PHA, Value::Implied) => {
                println!("PHA");

                let a = self.a;
                self.push8(a);

                Ok(0u8)
            }
            Instruction(Opcode::PLP, Value::Implied) => {
                println!("PLP");

                let mut res = self.pop8();
                res = res & 0b11001111;
                self.p_flags.bits = res;

                Ok(0u8)
            }
            Instruction(Opcode::ORA, Value::Immediate(val)) => {
                println!("ORA ${:02X}", val);
                
                self.a = self.a | val;

                let a = self.a;
                self.set_nz(a);

                Ok(0u8)
            }
            Instruction(Opcode::CLV, Value::Implied) => {
                println!("CLV");

                self.p_flags.remove(PFlag::FLAG_V);

                Ok(0u8)
            }
            Instruction(Opcode::EOR, Value::Immediate(val)) => {
                println!("EOR ${:02X}", val);

                self.a = self.a ^ val;

                let a = self.a;
                self.set_nz(a);

                Ok(0u8)
            }
            Instruction(Opcode::ADC, Value::Immediate(val)) => {
                println!("ADC ${:02X}", val);
                let ainit = self.a;

                let mut a: u16 = self.a as u16;
                a += val as u16;
                a += (self.p_flags.bits & 0b1) as u16;
                self.a = (a & 0xFF) as u8;

                if self.a == 0 {
                    self.p_flags.insert(PFlag::FLAG_Z);
                } else {
                    self.p_flags.remove(PFlag::FLAG_Z);
                }

                if (self.a as i8) < 0 {
                    self.p_flags.insert(PFlag::FLAG_N);
                } else {
                    self.p_flags.remove(PFlag::FLAG_N);
                }

                self.set_carry(a & 0x100 == 0x100);

                if (ainit ^ self.a) & (val ^ self.a) & 0x80 == 0x80 {
                    self.p_flags.insert(PFlag::FLAG_V);
                } else {
                    self.p_flags.remove(PFlag::FLAG_V);
                }

                Ok(0u8)
            }
            Instruction(Opcode::CPY, Value::Immediate(val)) => {
                println!("CPY #${:02X}", val);

                let t = self.y;
                let temp = self.y.wrapping_sub(val);
                self.set_nz(temp);
                self.set_carry(t >= val);

                Ok(0u8)
            }
            Instruction(Opcode::LDY, Value::Immediate(val)) => {
                println!("LDY #${:02X}", val);

                self.y = val;
                self.set_nz(val);

                Ok(0u8)
            }
            Instruction(Opcode::CPX, Value::Immediate(val)) => {
                println!("CPX #${:02X}", val);

                let t = self.x;
                let temp = self.x.wrapping_sub(val);
                self.set_nz(temp);
                self.set_carry(t >= val);

                Ok(0u8)
            }
            Instruction(Opcode::SBC, Value::Immediate(val)) => {
                println!("SBC ${:02X}", val);
                let ainit = self.a;
                let val = val ^ 0xFF;

                let mut a: u16 = self.a as u16;
                a += val as u16;
                a += (self.p_flags.bits & 0b1) as u16;
                self.a = (a & 0xFF) as u8;

                if self.a == 0 {
                    self.p_flags.insert(PFlag::FLAG_Z);
                } else {
                    self.p_flags.remove(PFlag::FLAG_Z);
                }

                if (self.a as i8) < 0 {
                    self.p_flags.insert(PFlag::FLAG_N);
                } else {
                    self.p_flags.remove(PFlag::FLAG_N);
                }

                self.set_carry(a & 0x100 == 0x100);

                if (ainit ^ self.a) & (val ^ self.a) & 0x80 == 0x80 {
                    self.p_flags.insert(PFlag::FLAG_V);
                } else {
                    self.p_flags.remove(PFlag::FLAG_V);
                }

                Ok(0u8)
            }
            Instruction(Opcode::INY, Value::Implied) => {
                println!("INY");

                let y = self.y;
                self.y = y.wrapping_add(1);

                let y = self.y;
                self.set_nz(y);

                Ok(0u8)
            }
            Instruction(Opcode::INX, Value::Implied) => {
                println!("INX");

                let x = self.x;
                self.x = x.wrapping_add(1);

                let x = self.x;
                self.set_nz(x);

                Ok(0u8)
            }
            Instruction(Opcode::DEY, Value::Implied) => {
                println!("DEY");

                let y = self.y;
                self.y = y.wrapping_sub(1);

                let y = self.y;
                self.set_nz(y);

                Ok(0u8)
            }
            Instruction(Opcode::DEX, Value::Implied) => {
                println!("DEX");

                let x = self.x;
                self.x = x.wrapping_sub(1);

                let x = self.x;
                self.set_nz(x);

                Ok(0u8)
            }
            Instruction(Opcode::TAY, Value::Implied) => {
                println!("TAY");

                let a = self.a;
                self.y = a;

                self.set_nz(a);

                Ok(0u8)
            }
            Instruction(Opcode::TAX, Value::Implied) => {
                println!("TAX");

                let a = self.a;
                self.x = a;

                self.set_nz(a);

                Ok(0u8)
            }
            Instruction(Opcode::TXA, Value::Implied) => {
                println!("TXA");

                let x = self.x;
                self.a = x;

                self.set_nz(x);

                Ok(0u8)
            }
            Instruction(Opcode::TYA, Value::Implied) => {
                println!("TYA");

                let y = self.y;
                self.a = y;

                self.set_nz(y);

                Ok(0u8)
            }
            Instruction(Opcode::TSX, Value::Implied) => {
                println!("TSX");

                let sp = self.sp;
                self.x = sp;

                self.set_nz(sp);

                Ok(0u8)
            }
            Instruction(Opcode::STX, Value::Absolute(addr)) => {
                println!("STX @{:04X}", addr);

                let x = self.x;
                self.write8(addr, x);

                Ok(0u8)
            }
            Instruction(Opcode::LDX, Value::Absolute(addr)) => {
                let val = self.read8(addr);
                println!("LDX @{:04X} = {:02X}", addr, val);
                self.x = val;

                self.set_nz(val);

                Ok(0u8)
            }
            Instruction(Opcode::RTI, Value::Implied) => {
                println!("RTI");

                let p = self.pop8();
                let ret = self.pop16();

                self.p_flags.bits = p;
                self.pc = ret;

                Ok(0u8)
            }
            Instruction(Opcode::LSR, Value::Implied) => {
                println!("LSR A");

                let mut a = self.a;
                self.set_carry(a & 0b1 == 0b1);
                a >>= 1;
                self.set_nz(a);
                self.a = a;
                
                Ok(0u8)
            }
            Instruction(Opcode::ASL, Value::Implied) => {
                println!("ASL A");

                let mut a = self.a;
                self.set_carry(a & 0x80 == 0x80);
                a <<= 1;
                self.set_nz(a);
                self.a = a;
                
                Ok(0u8)
            }
            Instruction(Opcode::ROR, Value::Implied) => {
                println!("ROR A");

                let mut a = self.a;
                let carry = a & 0b1 == 0b1;
                a >>= 1;
                if self.p_flags.contains(PFlag::FLAG_C) {
                    a |= 0x80;
                }
                self.set_nz(a);
                self.set_carry(carry);
                self.a = a;
                
                Ok(0u8)
            }
            Instruction(Opcode::ROL, Value::Implied) => {
                println!("ROL A");

                let mut a = self.a;
                let carry = a & 0x80 == 0x80;
                a <<= 1;
                if self.p_flags.contains(PFlag::FLAG_C) {
                    a |= 0b01;
                }
                self.set_nz(a);
                self.set_carry(carry);
                self.a = a;
                
                Ok(0u8)
            }
            Instruction(Opcode::LDA, Value::ZeroPage(offs)) => {
                let val = self.read8(offs as u16);
                println!("LDA @{:02X} = {:02X}", offs, val);
                self.a = val;

                self.set_nz(val);

                Ok(0u8)
            }
            instr => Err(format!("{:?}", instr)),
        }
    }

    pub fn set_carry(&mut self, val: bool) {
        match val {
            true => self.p_flags.insert(PFlag::FLAG_C),
            false => self.p_flags.remove(PFlag::FLAG_C),
        }
    }

    pub fn set_nz(&mut self, val: u8) { 
        match val {
            0 => self.p_flags.insert(PFlag::FLAG_Z),
            _ => self.p_flags.remove(PFlag::FLAG_Z),
        }

        match val & 0x80 {
            0x80 => self.p_flags.insert(PFlag::FLAG_N),
            _ => self.p_flags.remove(PFlag::FLAG_N),
        }
    }

    pub fn read8(&mut self, addr: u16) -> u8 {
        match addr {
            0x2002 => self.ppu.borrow().ppustatus,
            _ => self.mem.borrow_mut().read8(addr),
        }
    }

    pub fn read8_pc(&mut self) -> u8 {
        let ret = self.mem.borrow().read8(self.pc);
        self.pc += 1;
        ret
    }

    pub fn read16_pc(&mut self) -> u16 {
        let ret = (self.mem.borrow().read8(self.pc) as u16) | ((self.mem.borrow().read8(self.pc + 1) as u16) << 8);
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
            _ => self.mem.borrow_mut().write8(addr, val),
        }
    }

    fn push8(&mut self, val: u8) {
        self.mem.borrow_mut().write8(0x100 | self.sp as u16, val);
        self.sp -= 1;
    }

    fn push16(&mut self, val: u16) {
        self.push8((val >> 8) as u8);
        self.push8((val >> 0) as u8);
    }

    fn pop8(&mut self) -> u8 {
        self.sp += 1;
        let ret = self.mem.borrow().read8(0x100 | self.sp as u16);
        ret
    }

    fn pop16(&mut self) -> u16 {
        self.pop8() as u16 | ((self.pop8() as u16) << 8)
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    pub fn set_sp(&mut self, val: u8) {
        self.sp = val;
    }
}