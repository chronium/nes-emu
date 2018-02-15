use std::fmt;

#[derive(Debug, Clone)]
pub struct NESHeader {
    nes: [char; 4],
    prg_rom_sz: usize,
    chr_rom_sz: usize,
    flag_6: u8,
    flag_7: u8,
    prg_ram_sz: usize,
    flag_9: u8,
    flag_10: u8,
    zero: [u8; 5],
    pub mapper: u8,
}

impl From<Vec<u8>> for NESHeader {
    fn from(cart: Vec<u8>) -> Self {
        let mut nes = [0u8 as char; 4];
        nes[0] = cart[0] as char;
        nes[1] = cart[1] as char;
        nes[2] = cart[2] as char;
        nes[3] = cart[3] as char;

        let prg_rom = cart[4];
        let chr_rom = cart[5];
        let flag_6 = cart[6];
        let flag_7 = cart[7];
        let prg_ram = cart[8];
        let flag_9 = cart[9];
        let flag_10 = cart[10];
        let zero = [0u8; 5];

        let mapper = (flag_6 & 0xF0) >> 4 | (flag_7 & 0xF0);

        NESHeader {
            nes: nes,
            prg_rom_sz: prg_rom as usize * 16384,
            chr_rom_sz: chr_rom as usize * 8192,
            flag_6: flag_6,
            flag_7: flag_7,
            prg_ram_sz: prg_ram as usize * 8192,
            flag_9: flag_9,
            flag_10: flag_10,
            zero: zero,
            mapper: mapper,
        }
    }
}

impl<'a> From<&'a Vec<u8>> for NESHeader {
    fn from(cart: &'a Vec<u8>) -> Self {
        let mut nes = [0u8 as char; 4];
        nes[0] = cart[0] as char;
        nes[1] = cart[1] as char;
        nes[2] = cart[2] as char;
        nes[3] = cart[3] as char;

        let prg_rom = cart[4];
        let chr_rom = cart[5];
        let flag_6 = cart[6];
        let flag_7 = cart[7];
        let prg_ram = cart[8];
        let flag_9 = cart[9];
        let flag_10 = cart[10];
        let zero = [0u8; 5];

        let mapper = (flag_6 & 0xF0) >> 4 | (flag_7 & 0xF0);

        NESHeader {
            nes: nes,
            prg_rom_sz: prg_rom as usize * 16384,
            chr_rom_sz: chr_rom as usize * 8192,
            flag_6: flag_6,
            flag_7: flag_7,
            prg_ram_sz: prg_ram as usize * 8192,
            flag_9: flag_9,
            flag_10: flag_10,
            zero: zero,
            mapper: mapper,
        }
    }
}

#[derive(Clone)]
pub struct NESCart {
    pub header: NESHeader,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl fmt::Debug for NESCart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NESCart {{ header: {:?} }}", self.header)
    }
}

impl From<Vec<u8>> for NESCart {
    fn from(cart: Vec<u8>) -> Self {
        let header = NESHeader::from(&cart);

        let prg_size = match header.prg_rom_sz < 0x8000 {
            true => 0x8000,
            false => header.prg_rom_sz,
        };

        let mut prg_rom = vec![0u8; prg_size];
        let mut chr_rom = vec![0u8; header.chr_rom_sz];

        assert!(header.prg_rom_sz == cart[16..header.prg_rom_sz + 16].len());
        assert!(header.chr_rom_sz == cart[16 + header.prg_rom_sz..16 + header.prg_rom_sz + header.chr_rom_sz].len());

        if header.prg_rom_sz < 0x8000 {
            let mut tmp = Vec::<u8>::new();
            tmp.extend_from_slice(&cart[16..header.prg_rom_sz + 16]);
            tmp.extend_from_slice(&cart[16..header.prg_rom_sz + 16]);

            prg_rom.copy_from_slice(&tmp.as_slice());
        } else {
            prg_rom.copy_from_slice(&cart[16..header.prg_rom_sz + 16]);
        }
        chr_rom.copy_from_slice(&cart[16 + header.prg_rom_sz..16 + header.prg_rom_sz + header.chr_rom_sz]);

        NESCart {
            header: header,
            prg_rom: prg_rom,
            chr_rom: chr_rom,
        }
    }
}
