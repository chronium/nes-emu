use cart::NESCart;

#[derive(Debug)]
pub struct Memory<'a> {
    pub cart: &'a NESCart,
}

impl<'a> Memory<'a> {
    pub fn new(cart: &'a NESCart) -> Self {
        Memory {
            cart: cart
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x8000...0xBFFF => self.cart.prg_rom[addr as usize - 0x8000],
            0xC000...0xFFFF => self.cart.prg_rom[addr as usize - 0xC000],
            _ => unimplemented!()
        }
    }
}