#[derive(Debug, Clone)]
pub struct PPUCTL {
    nametable: BaseNameTable,
    vraminc: VRAMINC,
    spriteaddr: SpriteAddr,
    backaddr: BGPatternTableAddr,
    spritesize: SpriteSize,
    masterslave: MasterSlave,
    nmi: bool
}

impl From<u8> for PPUCTL {
    fn from(val: u8) -> Self {
        let nt = BaseNameTable::from(val);
        let vr = VRAMINC::from(val);
        let sa = SpriteAddr::from(val);
        let bg = BGPatternTableAddr::from(val);
        let ss = SpriteSize::from(val);
        let ms = MasterSlave::from(val);
        let nmi = match val & 0b10000000 {
            0b00000000 => false,
            0b10000000 => true,
            _ => panic!(),
        };

        Self {
            nametable: nt,
            vraminc: vr,
            spriteaddr: sa,
            backaddr: bg,
            spritesize: ss,
            masterslave: ms,
            nmi: nmi,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BaseNameTable {
    Zero,
    One,
    Two,
    Three,
}

impl From<BaseNameTable> for u16 {
    fn from(nt: BaseNameTable) -> Self {
        match nt {
            BaseNameTable::Zero => 0x2000,
            BaseNameTable::One => 0x2400,
            BaseNameTable::Two => 0x2800,
            BaseNameTable::Three => 0x2C00,
        }
    }
}

impl From<u8> for BaseNameTable {
    fn from(val: u8) -> Self {
        match val & 0b11 {
            0b00 => BaseNameTable::Zero,
            0b01 => BaseNameTable::One,
            0b10 => BaseNameTable::Two,
            0b11 => BaseNameTable::Three,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VRAMINC {
    Add1Across,
    Add32Down,
}

impl From<u8> for VRAMINC {
    fn from(val: u8) -> Self {
        match val & 0b100 {
            0b000 => VRAMINC::Add1Across,
            0b100 => VRAMINC::Add32Down,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SpriteAddr {
    Zero,
    One,
}

impl From<SpriteAddr> for u16 {
    fn from(addr: SpriteAddr) -> Self {
        match addr {
            SpriteAddr::Zero => 0x0000,
            SpriteAddr::One => 0x1000,
        }
    }
}

impl From<u8> for SpriteAddr {
    fn from(val: u8) -> Self {
        match val & 0b1000 {
            0b0000 => SpriteAddr::Zero,
            0b1000 => SpriteAddr::One,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BGPatternTableAddr {
    Zero,
    One,
}

impl From<BGPatternTableAddr> for u16 {
    fn from(val: BGPatternTableAddr) -> Self {
        match val {
            BGPatternTableAddr::Zero => 0x0000,
            BGPatternTableAddr::One => 0x1000,
        }
    }
}

impl From<u8> for BGPatternTableAddr {
    fn from(val: u8) -> Self {
        match val & 0b10000 {
            0b00000 => BGPatternTableAddr::Zero,
            0b10000 => BGPatternTableAddr::One,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SpriteSize {
    EightSq,
    Eight16,
}

impl From<u8> for SpriteSize {
    fn from(val: u8) -> Self {
        match val & 0b100000 {
            0b000000 => SpriteSize::EightSq,
            0b100000 => SpriteSize::Eight16,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MasterSlave {
    Master,
    Slave,
}

impl From<u8> for MasterSlave {
    fn from(val: u8) -> Self {
        match val & 0b1000000 {
            0b0000000 => MasterSlave::Master,
            0b1000000 => MasterSlave::Slave,
            _ => panic!(),
        }
    }
}
