use std::cell::RefCell;
use std::rc::Rc;

use ppuregs::PPUCTL;
use cart::NESCart;

#[derive(Debug, Clone)]
pub struct PPU {
    pub cart: Rc<RefCell<NESCart>>,
    pub ppuctl: PPUCTL,
}

impl PPU {
    pub fn new(cart: Rc<RefCell<NESCart>>, ppuctl: u8) -> Self {
        PPU {
            cart: cart,
            ppuctl: PPUCTL::from(ppuctl),
        }
    }
}

