use cart::NESCart;
use mem::Memory;
use cpu::NMOS6502;
use ppu::PPU;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct NES {
    pub cart: Rc<RefCell<NESCart>>,
    pub mem: Memory,
    pub cpu: NMOS6502,
    pub ppu: Rc<RefCell<PPU>>,
}

impl NES {
    pub fn new(cart: Rc<RefCell<NESCart>>) -> Self {
        let mem = Memory::new(cart.clone());
        let ppu = Rc::new(RefCell::new(PPU::new(cart.clone(), 0u8)));
        let cpu = NMOS6502::new(box mem, ppu.clone());

        NES {
            cart: cart.clone(),
            mem: Memory {
                cart: cart.clone()
            },
            cpu: cpu,
            ppu: ppu,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn step(&mut self) -> Result<u8, String> {
        self.cpu.step()
    }

    pub fn run(&mut self) {
        self.step();
    }
}
