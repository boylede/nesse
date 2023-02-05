use crate::prelude::*;
/// nes ppu instance
pub struct Nes2c02 {
    pub controller: u8,
    pub mask: u8,
    pub status: u8,
    pub oam_address: u8,
    pub oam_data: u8,
    pub scroll: u8,
    pub address: AddressRegisterLatch,
    pub data: u8,
    pub oam_dma: u8,
    // 0, 1, 2, or 3 to indicate syncronization between cpu and ppu clocks
    pub timing: u8,
    pub clock_counter: u8,
    pub frame_clock: u16,
    pub pallete_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam: [u8; 256],
}

pub enum AddressRegisterLatch {
    Unset,
    First(u8),
    Second(u8, u8),
}

enum DataRegisterLatch {
    Stale(u8),
    Fresh(u8),
}

impl Nes2c02 {
    pub fn tick(&mut self, _cart: &mut Option<NesCart>) {
        // tick
    }
}

pub const PPU_ORIGIN: u16 = 0x2000;
pub const PPU_END: u16 = 0x4020;

pub const PPU_CONTROLLER: u16 = 0x2000;
pub const PPU_MASK: u16 = 0x2001;
pub const PPU_STATUS: u16 = 0x2002;
pub const PPU_OAM_ADDRESS: u16 = 0x2003;
pub const PPU_OAM_DATA: u16 = 0x2004;
pub const PPU_SCROLL: u16 = 0x2005;
pub const PPU_ADDRESS: u16 = 0x2006;
pub const PPU_DATA: u16 = 0x2007;
pub const PPU_OAM_DMA: u16 = 0x4014;

impl AddressableMemory for Nes2c02 {
    fn bounds(&self) -> (u16, u16) {
        unimplemented!()
    }
    fn set(&mut self, address: u16, value: u8) {
        if address < PPU_CONTROLLER {
            // nes base ram
        } else if address < PPU_END {
            // other hardware
            println!(
                "837: wanted to write {:02X} to address {:04X}",
                value, address
            );
            match address {
                PPU_CONTROLLER => {
                    // controller
                    self.controller = value;
                }
                PPU_MASK => {
                    // mask
                    self.mask = value;
                }
                PPU_STATUS => {
                    // status
                    self.status = value;
                }
                PPU_OAM_ADDRESS => {
                    // oam address
                    self.oam_address = value;
                }
                PPU_OAM_DATA => {
                    // oam data
                    self.oam_data = value;
                }
                PPU_SCROLL => {
                    // scroll
                    self.scroll = value;
                }
                PPU_ADDRESS => {
                    // address
                    match self.address {
                        AddressRegisterLatch::Unset => {
                            self.address = AddressRegisterLatch::First(value);
                        }
                        AddressRegisterLatch::First(hi) => {
                            self.address = AddressRegisterLatch::Second(hi, value);
                        }
                        AddressRegisterLatch::Second(_, _) => {
                            self.address = AddressRegisterLatch::First(value);
                        }
                    }
                }
                PPU_DATA => {
                    // data not writable
                }
                PPU_OAM_DMA => {
                    // oam dma
                    self.oam_dma = value;
                }
                _ => {
                    unimplemented!()
                }
            }
        } else {
            // other addresses handled by cartridge
            unimplemented!()
        }
    }
    fn get(&self, address: u16) -> u8 {
        if address < PPU_ORIGIN {
            unimplemented!()
        } else if address < PPU_END {
            // other hardware
            println!("849: wanted to read address {:04X}", address);
            0
        } else {
            // other addresses handled by cartridge
            unimplemented!()
        }
    }
}

impl Default for Nes2c02 {
    fn default() -> Nes2c02 {
        Nes2c02 {
            controller: 0u8,
            mask: 0u8,
            status: 0u8,
            oam_address: 0u8,
            oam_data: 0u8,
            scroll: 0u8,
            address: AddressRegisterLatch::Unset,
            data: 0,
            oam_dma: 0u8,
            timing: 0u8,
            clock_counter: 0u8,
            frame_clock: 0u16,
            pallete_table: [0u8; 32],
            vram: [0u8; 2048],
            oam: [0u8; 256],
        }
    }
}
