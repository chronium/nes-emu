use cart::NESCart;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Memory {
    pub cart: Rc<RefCell<NESCart>>,
}

impl Memory {
    pub fn new(cart: Rc<RefCell<NESCart>>) -> Self {
        Memory {
            cart: cart
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x8000...0xBFFF => self.cart.borrow_mut().prg_rom[addr as usize - 0x8000],
            0xC000...0xFFFF => self.cart.borrow_mut().prg_rom[addr as usize - 0xC000],
            _ => unimplemented!()
        }
    }
}