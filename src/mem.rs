use cart::NESCart;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Memory {
    pub cart: Arc<Mutex<NESCart>>,
    pub ram: [u8; 0x800],
}

impl Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory")
    }
}

impl Memory {
    pub fn new(cart: Arc<Mutex<NESCart>>) -> Self {
        Memory {
            cart: cart,
            ram: [0u8; 0x800],
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        let cart = self.cart.lock().unwrap();
        let mapper = cart.header.mapper;
        match addr {
            0x0000...0x2000 => self.ram[addr as usize % 0x800],
            0x8000...0xFFFF =>
                match mapper {
                    0 => match addr {
                        0x8000...0xBFFF => cart.prg_rom[addr as usize - 0x8000],
                        0xC000...0xFFFF => cart.prg_rom[addr as usize - 0x8000],
                        _ => panic!("Cannot read addr: 0x{:X}", addr)
                    }
                    _ => panic!("Unimplemented mapper: {}", mapper)
                }
            _ => panic!("Cannot read addr: 0x{:X}", addr)
        }
   }

    pub fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x2000 => self.ram[addr as usize % 0x800] = val,
            0x4017 => {}
            _ => panic!("Cannot write addr: 0x{:X}", addr)
        }
    }
}