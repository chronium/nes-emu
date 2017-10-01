use std::cell::RefCell;
use std::rc::Rc;

use ppuregs::PPUCTL;
use cart::NESCart;

const HEIGHT: u16 = 261;
const WIDTH: u16 = 340; 

#[derive(Debug, Clone)]
pub struct PPU {
    pub cart: Rc<RefCell<NESCart>>,
    pub ppuctl: PPUCTL,
    pub y: u16,
    pub x: u16,
    pub ppustatus: u8,
}

impl PPU {
    pub fn new(cart: Rc<RefCell<NESCart>>, ppuctl: u8) -> Self {
        PPU {
            cart: cart,
            ppuctl: PPUCTL::from(ppuctl),
            y: 0u16,
            x: 0u16,
            ppustatus: 0u8,
        }
    }

    pub fn step(&mut self) {
        if self.y == 241 {
            self.ppustatus |= 0x80;
        }

        self.x += 1;
        if self.x >= WIDTH {
            self.y += 1;
            self.x = 0;
        }

        if self.y >= HEIGHT {
            self.y = 0;
            self.ppustatus &= !0x80;
        }
    }
}

