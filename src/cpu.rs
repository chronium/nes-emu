use inst::Instruction;
use mem::Memory;

bitflags! {
    #[derive(Default)]
    pub struct PFlag: u8 {
        const NONE   = 0b00000000;
        const FLAG_C = 0b00000001;
        const FLAG_Z = 0b00000010;
        const FLAG_I = 0b00000100;
        const FLAG_D = 0b00001000;
        const FLAG_B = 0b00010000;
        const FLAG_V = 0b01000000;
        const FLAG_S = 0b10000000;
    }
}

#[derive(Debug)]
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
            p_flags: PFlag::NONE,
        }
    }

    pub fn reset(&mut self) {
        self.pc = (self.mem.read8(0xFFFD) as u16) << 8 | (self.mem.read8(0xFFFC) as u16);
    }

    pub fn step(&mut self) {
        print!("0x{:4X}: ", self.pc);
        match Instruction::from(self) {
            instr => println!("{:?}", instr),
        }
    }

    pub fn read8_pc(&mut self) -> u8 {
        let ret = self.mem.read8(self.pc);
        self.pc += 1;
        ret
    }
}