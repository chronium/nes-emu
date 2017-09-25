#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate bitflags;

extern crate minifb;

use minifb::{Key, WindowOptions, Window, Scale};

mod cart;
mod inst;
mod nes;
mod mem;
mod cpu;

use cart::NESCart;
use nes::NES;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use std::cell::RefCell;
use std::rc::Rc;

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

fn main() {
    let matches = clap_app!(snes_emu =>
        (version: VERSION)
        (author: AUTHORS)
        (about: "Rustic NES Emulator")
        (@arg INPUT: +required "ROM file to load")
    ).get_matches();

    let rom_path = matches.value_of("INPUT").unwrap();
    println!("Opening ROM: {}", rom_path);

    let mut rom_raw = Vec::<u8>::new();

    File::open(rom_path).and_then(|mut f| f.read_to_end(&mut rom_raw)).unwrap();

    let cart = NESCart::from(rom_raw);

    let nes = &mut NES::new(Rc::new(RefCell::new(cart)));
    unsafe { 
        nes.reset();
        nes.run();
    }

    //println!("{:?}", &nes.cpu);

    /*let sprite_bytes = [0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x7Eu8, 0x3Cu8,
                        0x3Cu8, 0x7Eu8, 0x7Eu8, 0xFFu8, 0xFFu8, 0xFFu8, 0x42u8, 0x00u8];

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new("NES Emulator",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions {
                                    scale: Scale::X4,
                                    ..Default::default()
                                }).unwrap_or_else(|e| {
                                     panic!("{}", e);
                                 });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..240 {
            for x in 0..240 {
                let col = ((((sprite_bytes[y % 8] & 0xFF) >> (x % 8)) as u8) & 0b1) | (((((sprite_bytes[(y % 8) + 8] & 0xFF) >> (x % 8)) as u8) & 0b1) << 1);
                buffer[x + y * WIDTH] = palette[col as usize + x / 8];
            }
        }

        window.update_with_buffer(&buffer).unwrap();
    }*/
}
