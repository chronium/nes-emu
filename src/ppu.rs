use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

use std::fmt;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use ppuregs::{PPUCTL, VRAMINC};
use cart::NESCart;
use cpu::NMOS6502;
use mem::Memory;

const HEIGHT: u16 = 261;
const WIDTH: u16 = 340;

#[derive(Clone)]
pub struct PPU {
    pub cart: Arc<Mutex<NESCart>>,
    pub mem: Arc<Mutex<Memory>>,
    pub ppuctl: PPUCTL,
    pub ppuaddr: u16,
    pub y: u16,
    pub x: u16,
    pub ppustatus: u8,
    pub vram: [u8; 0x4000],
    pub w: u8,
    pub xscroll: u8,
    pub yscroll: u8,
    pub oamaddr: u8,
    pub oam: [u8; 0xFF],
    pub cycles: u64,
}

impl Debug for PPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPU")
    }
}

impl PPU {
    pub fn new(cart: Arc<Mutex<NESCart>>, mem: Arc<Mutex<Memory>>, ppuctl: u8) -> Self {
        PPU {
            cart: cart,
            mem: mem,
            ppuctl: PPUCTL::from(ppuctl),
            ppuaddr: 0u16,
            y: 0u16,
            x: 0u16,
            ppustatus: 0u8,
            vram: [0u8; 0x4000],
            w: 0u8,
            xscroll: 0u8,
            yscroll: 0u8,
            oamaddr: 0u8,
            oam: [0u8; 0xFF],
            cycles: 0u64,
        }
    }

    pub fn step(&mut self, cpu: Arc<Mutex<NMOS6502>>) {
        self.cycles += 1;

        if self.cycles > 29658 {
            if self.y == 241 {
                self.ppustatus |= 0x80;
                if self.x == 1 && self.ppuctl.nmi {
                    let mut cpu = cpu.lock().unwrap();
                    cpu.nmi();
                }
            }

            if self.y >= 257 && self.y <= 320 {
                self.oamaddr = 0;
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

    pub fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            0x2000 => self.ppuctl = PPUCTL::from(val),
            0x2003 => self.oamaddr = val,
            0x2001 => println!("PPUMASK {:X}", val),
            0x2005 => {
                let w = self.w;
                match w {
                    0 => {
                        self.xscroll = val;
                        self.w = 1;
                    }
                    _ => {
                        self.yscroll = val;
                        self.w = 0;
                    }
                }
            }
            0x2006 => {
                let w = self.w;
                match w {
                    0 => {
                        self.ppuaddr = ((val as u16) << 8) | (self.ppuaddr & 0x00FF); 
                        self.w = 1;
                    }
                    _ => {
                        self.ppuaddr = (self.ppuaddr & 0xFF00) | (val as u16); 
                        self.w = 0;
                    }
                }
            }
            0x2007 => {
                if self.ppustatus == 0x80 {
                    let addr = self.ppuaddr as usize;
                    self.vram[addr] = val;

                    match self.ppuctl.vraminc {
                        VRAMINC::Add1Across => self.ppuaddr += 1,
                        VRAMINC::Add32Down => self.ppuaddr += 32,
                    }
                }
            }
            _ => panic!("Cannot write to PPU @{:04X} = {:02X}", addr, val),
        }
    }

    pub fn oamdma(&mut self, port: u8) {
        println!("OAMDMA @{:02X}00 => @{:02X}", port, self.oamaddr);

        let from = (port as u16) << 8;

        let mem = self.mem.lock().unwrap();

        for i in 0..0xFF {
            self.oam[(self.oamaddr + i) as usize] = mem.read8(from + i as u16);
        }
    }
}

