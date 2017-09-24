use cart::NESCart;
use mem::Memory;
use cpu::NMOS6502;

#[derive(Debug)]
pub struct NES<'a> {
    pub cart: &'a NESCart,
    pub mem: Memory<'a>,
    pub cpu: NMOS6502<'a>
}

impl<'a> NES<'a> {
    pub fn new(cart: &'a NESCart) -> Self {
        let mem = Memory::new(cart);
        let cpu = NMOS6502::new(box mem);

        NES {
            cart: cart,
            mem: Memory {
                cart: cart
            },
            cpu: cpu
        }
    }

    pub unsafe fn reset(&mut self) {
        self.cpu.reset();
    }

    pub unsafe fn step(&'a mut self) {
        self.cpu.step();
    }

    pub unsafe fn run(&'a mut self) {
        loop {
            self.step();
        };
    }
}
