use cart::NESCart;
use mem::Memory;
use cpu::NMOS6502;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct NES {
    pub cart: Rc<RefCell<NESCart>>,
    pub mem: Memory,
    pub cpu: NMOS6502
}

impl NES {
    pub fn new(cart: Rc<RefCell<NESCart>>) -> Self {
        let mem = Memory::new(cart.clone());
        let cpu = NMOS6502::new(box mem);

        NES {
            cart: cart.clone(),
            mem: Memory {
                cart: cart.clone()
            },
            cpu: cpu
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn run(&mut self) {
        self.step();
    }
}
