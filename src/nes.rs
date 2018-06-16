use cart::NESCart;
use mem::Memory;
use cpu::NMOS6502;
use ppu::PPU;

use std::cell::RefCell;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct NES {
    pub cart: Arc<RwLock<NESCart>>,
    pub mem: Arc<Mutex<Memory>>,
    pub cpu: Arc<Mutex<NMOS6502>>,
    pub ppu: Arc<Mutex<PPU>>,
    pub kill: bool,
}

impl NES {
    pub fn new(cart: Arc<RwLock<NESCart>>) -> Self {
        let mem = Arc::new(Mutex::new(Memory::new(cart.clone())));
        let ppu = Arc::new(Mutex::new(PPU::new(cart.clone(), mem.clone(), 0u8)));
        let cpu = Arc::new(Mutex::new(NMOS6502::new(mem.clone(), ppu.clone())));

        NES {
            cart: cart.clone(),
            mem: mem,
            cpu: cpu,
            ppu: ppu,
            kill: false,
        }
    }

    pub fn reset(&mut self) {
        let mut cpu = self.cpu.lock().unwrap();
        cpu.reset();
    }

    pub fn step(&mut self) -> Result<u8, String> {
        let mut cpu = self.cpu.lock().unwrap();
        let mut res = cpu.step();
        let mut ppu = self.ppu.lock().unwrap();
        ppu.step(self.cpu.clone());
        ppu.step(self.cpu.clone());
        ppu.step(self.cpu.clone());

        if self.kill {
            res = Err(String::from("Ended"));
        }

        res
    }

    pub fn run(&mut self) {
        self.step();
    }
}
