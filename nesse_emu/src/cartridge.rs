use crate::{prelude::*, mapper::Mapper};

/// a cartridge as described by a iNes 1.0 or 2.0 file (2.0 currently unimplemented)
pub struct NesCart {
    pub header: NesCartHeader,
    trainer: Option<Vec<u8>>,
    /// program memory
    prg_rom: Vec<u8>,
    /// character memory / pattern memory
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    mapper: Box<dyn Mapper>,
}

pub struct NesCartHeader {
    mapper_id: u8,
    mirroring: u8,
    four_screen: bool,
    battery: bool,
}

impl NesCart {
    pub fn from_slice(mut bytes: &[u8]) -> Option<NesCart> {
        // let mut buffer = bytes.to_vec();

        let mut header: [u8; 16] = [0u8; 16];
        bytes.read_exact(&mut header).unwrap();

        // let mut buffer: [u8; 16] = [0u8; 16];
        // buffer.copy_from_slice(&bytes[0..16]);
        let sigil: [u8; 4] = [header[0], header[1], header[2], header[3]];
        if sigil != [0x4e, 0x45, 0x53, 0x1a] {
            println!("nes rom sigil not found");
            return None;
        }
        let rom_count = header[4];
        let vrom_count = header[5];
        let control_bytes: [u8; 2] = [header[6], header[7]];
        let ram_count = header[8];
        let _reserved = header[9];
        let reserved_zeros: [u8; 6] = [
            header[10], header[11], header[12], header[13], header[14], header[15],
        ];
        if reserved_zeros != [0, 0, 0, 0, 0, 0] {
            println!("unexpected values reserved area of rom header:");
            println!(
                "{} {} {} {} {} {}",
                char::from(reserved_zeros[0]),
                char::from(reserved_zeros[1]),
                char::from(reserved_zeros[2]),
                char::from(reserved_zeros[3]),
                char::from(reserved_zeros[4]),
                char::from(reserved_zeros[5]),
            );
        }
        // println!("number of 16kB rom banks: {}", rom_count);
        // println!("number of 8kB vrom banks: {}", vrom_count);
        // flags in control byte 0 (aka 1 in references)
        const FLAG_MIRRORING: u8 = 1 << 0;
        const FLAG_BBRAM: u8 = 1 << 1;
        const FLAG_TRAINER: u8 = 1 << 2;
        const FLAG_FOUR_SCREEN: u8 = 1 << 3;
        // flags in control byte 1 (aka 2 in references)
        const FLAG_RESERVED_0: u8 = 1 << 0;
        const FLAG_RESERVED_1: u8 = 1 << 1;
        const FLAG_RESERVED_2: u8 = 1 << 2;
        const FLAG_RESERVED_3: u8 = 1 << 3; // one if iNES2.0

        let mapper_id = {
            let upper = control_bytes[1] & 0xf0;
            let lower = control_bytes[0] & 0xf0;
            upper | lower >> 4
        };

        let mirroring = control_bytes[0] & FLAG_MIRRORING;
        // println!("mirroring mode: {}", mirroring);
        let battery = control_bytes[0] & FLAG_BBRAM > 0; // at 0x6000..0x7fff
                                                         // println!("has battery: {}", battery);
        let has_trainer = control_bytes[0] & FLAG_TRAINER > 0;
        // println!("has trainer: {}", has_trainer);
        let four_screen = (control_bytes[0] & FLAG_FOUR_SCREEN) > 0;
        // println!("uses four screen mirroring: {}", four_screen);
        // println!("expects mapper {}", mapper_id);
        // println!("number of 8kB ram banks: {}", ram_count);
        // println!("other byte: {:x}", reserved);

        let unhandled_bits = FLAG_RESERVED_0 | FLAG_RESERVED_1 | FLAG_RESERVED_2 | FLAG_RESERVED_3;
        let unhandled = control_bytes[1] & unhandled_bits;

        if unhandled != 0 {
            println!(
                "has unexpected items in control byte 2, may be different file version: {:x}",
                unhandled
            );
            return None;
        }
        let header = NesCartHeader {
            mapper_id,
            mirroring,
            four_screen,
            battery,
        };

        let trainer = if has_trainer {
            let mut t = vec![0u8; 512];
            bytes.read_exact(&mut t).unwrap();
            // println!("  -trainer = {} bytes", bytes.len());
            Some(t)
        } else {
            None
        };

        let prg_rom_size = rom_count as usize * 16 * 1024;
        // println!("getting {} bytes for prg_rom", prg_rom_size);
        let mut prg_rom = vec![0u8; prg_rom_size];
        if bytes.read_exact(&mut prg_rom).is_err() {
            println!("file ended early while reading program rom. expected {rom_count} roms for {prg_rom_size} bytes");
            return None;
        }
        // println!("retreived {} bytes for prg_rom", prg_rom.len());
        // println!("  -prg_rom = {} bytes", bytes.len());
        let chr_rom_size = vrom_count as usize * 8 * 1024;
        let mut chr_rom = vec![0u8; chr_rom_size];
        bytes.read_exact(&mut chr_rom).unwrap();
        // println!("retreived {} bytes for chr_rom", chr_rom.len());
        // println!("  -chr_rom = {} bytes", bytes.len());

        let prg_ram = if battery {
            // todo: do we load in battery-backed ram from another file?
            unimplemented!()
        } else {
            Vec::with_capacity(8 * 1024 * ram_count as usize)
        };

        let mapper = Box::new(());

        Some(NesCart {
            header,
            trainer,
            prg_rom,
            chr_rom,
            prg_ram,
            mapper,
        })
    }
}

impl Bus for NesCart {
    fn bounds(&self) -> (u16, u16) {
        (0x4020, 0xffff)
    }
    fn set(&mut self, address: u16, value: u8) {
        let address = self.mapper.translate(address);

        if address < 0x6000 {
            // special depending on cartridge generation
            unimplemented!()
        } else if address < 0x8000 {
            // optional ram, for e.g. zelda
            unimplemented!()
        } else {
            // cartridge rom
            // todo: does any cartridge even try?
            println!(
                "tried to set value {} at address {} in cartridge rom",
                value, address
            );
        }
    }
    fn get(&mut self, address: u16) -> u8 {
        let address = self.mapper.translate(address);
        // todo: bounds checks
        if address < 0x6000 {
            // special depending on cartridge generation
            unimplemented!()
        } else if address < 0x8000 {
            // optional ram, for e.g. zelda
            println!("tried getting option ram address {:04X}", address);
            unimplemented!()
        } else {
            // cartridge rom
            let mut rom_address = address - 0x8000;
            // println!("getting cartridge rom address {:x} translates to {:x}", address, rom_address);
            if rom_address >= self.prg_rom.len() as u16 {
                rom_address -= 0x4000;
                // println!("subtracting 0x4000 = {:x}", rom_address);
            }
            self.prg_rom[rom_address as usize]
        }
    }
}
