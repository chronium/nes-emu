use cart::NESCart;

use std::cell::RefCell;
use std::rc::Rc;

use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Memory {
    pub cart: Rc<RefCell<NESCart>>,
    pub ram: [u8; 0x800],
}

impl Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory")
    }
}

impl Memory {
    pub fn new(cart: Rc<RefCell<NESCart>>) -> Self {
        Memory {
            cart: cart,
            ram: [0u8; 0x800],
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        let mapper = self.cart.borrow().header.mapper;
        match mapper {
            0 => {
                match addr {
                    0x8000...0xBFFF => self.cart.borrow_mut().prg_rom[addr as usize - 0x8000],
                    0xC000...0xFFFF => self.cart.borrow_mut().prg_rom[addr as usize - 0xC000],
                    _ => panic!("addr: 0x{:X}", addr)
                }
            },
            1 => {
                match addr {
                    0x8000...0xBFFF => self.cart.borrow_mut().prg_rom[addr as usize - 0x8000],
                    0xC000...0xFFFF => self.cart.borrow_mut().prg_rom[addr as usize - 0x8000],
                    _ => panic!("addr: 0x{:X}", addr)
                }
            }
            _ => panic!("Unimplemented mapper: {}", mapper)
        }
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x2000 => self.ram[addr as usize % 0x800] = val,
            _ => panic!("Cannot write addr: 0x{:X}", addr)
        }
    }
}