use cart::NESCart;
use mem::Memory;
use cpu::NMOS6502;
use ppu::PPU;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct NES {
    pub cart: Rc<RefCell<NESCart>>,
    pub mem: Rc<RefCell<Memory>>,
    pub cpu: NMOS6502,
    pub ppu: Rc<RefCell<PPU>>,
}

impl NES {
    pub fn new(cart: Rc<RefCell<NESCart>>) -> Self {
        let mem = Rc::new(RefCell::new(Memory::new(cart.clone())));
        let ppu = Rc::new(RefCell::new(PPU::new(cart.clone(), mem.clone(), 0u8)));
        let cpu = NMOS6502::new(mem.clone(), ppu.clone());

        NES {
            cart: cart.clone(),
            mem: mem,
            cpu: cpu,
            ppu: ppu,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn step(&mut self) -> Result<u8, String> {
        let res = self.cpu.step();
        self.ppu.borrow_mut().step();
        self.ppu.borrow_mut().step();
        self.ppu.borrow_mut().step();

        res
    }

    pub fn run(&mut self) {
        self.step();
    }
}
