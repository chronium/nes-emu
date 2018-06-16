#![feature(box_syntax, box_patterns, inclusive_range_syntax)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

extern crate clock_ticks;
extern crate minifb;

mod ppuregs;
mod cart;
mod inst;
mod nes;
mod mem;
mod cpu;
mod ppu;

use cart::NESCart;
use nes::NES;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use std::cell::RefCell;
use std::rc::Rc;

use std::sync::{Arc, Mutex};

use std::thread;
use std::time::Duration;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

const palette: [u32; 64] = [
     0x7C7C7Cu32 ,0x0000FCu32 ,0x0000BCu32 ,0x4428BCu32 ,0x940084u32 ,0xA80020u32 ,0xA81000u32 ,0x881400u32
    ,0x503000u32 ,0x007800u32 ,0x006800u32 ,0x005800u32 ,0x004058u32 ,0x000000u32 ,0x000000u32 ,0x000000u32
    ,0xBCBCBCu32 ,0x0078F8u32 ,0x0058F8u32 ,0x6844FCu32 ,0xD800CCu32 ,0xE40058u32 ,0xF83800u32 ,0xE45C10u32
    ,0xAC7C00u32 ,0x00B800u32 ,0x00A800u32 ,0x00A844u32 ,0x008888u32 ,0x000000u32 ,0x000000u32 ,0x000000u32
    ,0xF8F8F8u32 ,0x3CBCFCu32 ,0x6888FCu32 ,0x9878F8u32 ,0xF878F8u32 ,0xF85898u32 ,0xF87858u32 ,0xFCA044u32 
    ,0xF8B800u32 ,0xB8F818u32 ,0x58D854u32 ,0x58F898u32 ,0x00E8D8u32 ,0x787878u32 ,0x000000u32 ,0x000000u32
    ,0xFCFCFCu32 ,0xA4E4FCu32 ,0xB8B8F8u32 ,0xD8B8F8u32 ,0xF8B8F8u32 ,0xF8A4C0u32 ,0xF0D0B0u32 ,0xFCE0A8u32
    ,0xF8D878u32 ,0xD8F878u32 ,0xB8F8B8u32 ,0xB8F8D8u32 ,0x00FCFCu32 ,0xF8D8F8u32 ,0x000000u32 ,0x000000u32
];

const WIDTH: usize = 355;
const HEIGHT: usize = 240;

use minifb::{Key, WindowOptions, Window, Scale};

pub fn cpu_loop(rate: u64, nes: Arc<Mutex<NES>>) {
    let thr = thread::spawn(move || {
        let mut accumulator = 0;
        let mut previous_clock = clock_ticks::precise_time_ns();

        let rate = 1_000_000_000 / rate;

        loop {
            match nes.lock().unwrap().step() {
                Result::Err(f) => break,
                Result::Ok(f) => (),
            };

            let now = clock_ticks::precise_time_ns();
            accumulator += now - previous_clock;
            previous_clock = now;

            while accumulator >= rate {
                accumulator -= rate;
            }

            thread::sleep(Duration::from_millis(((rate - accumulator) / 1000000) as u64));
        }
    });
}

fn main() {
    let matches = clap_app!(snes_emu =>
        (version: VERSION)
        (author: AUTHORS)
        (about: "Rustic NES Emulator")
        (@arg INPUT: +required "ROM file to load")
        (@arg pc: -p +takes_value "Set PC execution start")
        (@arg sp: -s +takes_value "Set SP execution start")
    ).get_matches();

    let rom_path = matches.value_of("INPUT").unwrap();
    println!("Opening ROM: {}", rom_path);

    let mut rom_raw = Vec::<u8>::new();

    File::open(rom_path).and_then(|mut f| f.read_to_end(&mut rom_raw)).unwrap();

    let cart = NESCart::from(rom_raw);

    let nes = &mut Arc::new(Mutex::new(NES::new(Arc::new(RwLock::new(cart)))));
    let ness = &mut nes.clone();
    ness.lock().unwrap().reset();

    if matches.is_present("pc") {
        let ness = nes.lock().unwrap();
        let mut cpu = ness.cpu.lock().unwrap();
        cpu.set_pc(u16::from_str_radix(matches.value_of("pc").unwrap(), 16).unwrap());
    }

    if matches.is_present("sp") {
        let ness = nes.lock().unwrap();
        let mut cpu = ness.cpu.lock().unwrap();
        cpu.set_sp(u8::from_str_radix(matches.value_of("sp").unwrap(), 16).unwrap());
    }

    let nes_arc = nes.clone();
    cpu_loop (1000, nes_arc);

    let mut window = Window::new("NES Emulator", WIDTH as usize,
                                HEIGHT as usize,
                                WindowOptions {
                                    scale: Scale::X4,
                                    ..Default::default()
                                }).unwrap_or_else(|e| {
                                     panic!("{}", e);
                                });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut sprite_bytes: [u8; 256*16] = [0u8; 256*16];
    let ness = nes.lock().unwrap();
    let cart = ness.cart.lock().unwrap();
        panic!("wtf");
    sprite_bytes.copy_from_slice(&cart.chr_rom[0..256*16]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let nesl = nes.lock();
        let nesu = nesl.unwrap();
        let ppul = nesu.ppu.lock();
        let ppuu = ppul.unwrap();
        let vram = ppuu.vram;
        panic!("wtf");
        for y in 0..8usize {
            for x in 0..8usize {
                let ind: usize = vram[0x21ca] as usize * 16;
                let b1 = ((((sprite_bytes[ind + (y % 8) + 0] as u8) >> (7 - (x % 8))) & 0b1) << 0) as u8;
                let b2 = ((((sprite_bytes[ind + (y % 8) + 8] as u8) >> (7 - (x % 8))) & 0b1) << 1) as u8;
                let col = b1 | b2;
                buffer[x as usize + y as usize * WIDTH] = palette[col as usize + 32];
            }
        }

        window.update_with_buffer(&buffer);
    }

    nes.lock().unwrap().kill = true;

    println!("{:?}", &nes.lock().unwrap().cpu);
}
