use crate::prelude::*;
/// nes ppu instance
pub struct Nes2c02 {
    /// the read/write bus current value
    latch: DecayingLatch,
    /// the internal status of the control register
    control: ControlRegister,
    /// silly, but some behaviour is temperture dependant
    temp: Temperature,
    /// are we in v_blank right now
    v_blank: bool,
    mask: MaskRegister,
    status: StatusRegister,
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


/// this is really silly but I am adding it anyway
/// will probably delete later
enum Temperature {
    Cold(u32),
    Warm,
}

impl Temperature {
    fn tick(&mut self) {
        use Temperature::*;
        match self {
            Temperature::Cold(count) => {
                *self = if *count > 30_000 {
                    Warm
                } else {
                    // todo: this might need to be incremented more than 1 per tick
                    Cold(*count + 1)
                }
            },
            Warm => todo!(),
        }
    }
    fn is_warm(&self) -> bool {
        use Temperature::*;
        match self {
            Cold(_) => false,
            Warm => true,
        }
    }
}


#[derive(Copy, Clone)]
enum DecayingLatch {
    Decaying(u8),
    Stable(u8, u8),
}

impl DecayingLatch {
    fn read(&self) -> u8 {
        match self {
            DecayingLatch::Decaying(l) => *l,
            DecayingLatch::Stable(l, _) => *l,
        }
    }
    fn write(&mut self, value: u8) {
        *self = DecayingLatch::Stable(value, 3);
    }
    fn decay(&mut self) {
        use DecayingLatch::*;
        *self = match self {
            Decaying(l) => Decaying(*l),
            Stable(l, counter) => {
                if *counter > 0 {
                    Stable(*l, *counter - 1)
                } else {
                    Decaying(*l)
                }
            }
        }
    }
}

pub struct PpuBus<'a, 'b> {
    ppu: &'a mut Nes2c02,
    cart: &'b mut NesCart,
}

impl<'a, 'b> Bus for PpuBus<'a, 'b> {
    fn bounds(&self) -> (u16, u16) {
        todo!()
    }

    fn set(&mut self, address: u16, value: u8) {
        todo!()
    }

    fn get(&mut self, address: u16) -> u8 {
        todo!()
    }
}

impl Nes2c02 {
    pub fn ppu_bus<'a, 'b>(&'a mut self, cart: &'b mut NesCart) -> PpuBus<'a, 'b> {
        PpuBus { ppu: self, cart }
    }
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
        
        self.temp.tick();
        // tick

        self.latch.decay();
    }
}

pub const PPU_ORIGIN: u16 = 0x2000;
pub const PPU_END: u16 = 0x4020;

pub const PPU_CONTROLLER: u16 = 0x000;
pub const PPU_MASK: u16 = 0x001;
pub const PPU_STATUS: u16 = 0x002;
pub const PPU_OAM_ADDRESS: u16 = 0x003;
pub const PPU_OAM_DATA: u16 = 0x004;
pub const PPU_SCROLL: u16 = 0x005;
pub const PPU_ADDRESS: u16 = 0x006;
pub const PPU_DATA: u16 = 0x007;

pub const PPU_OAM_DMA: u16 = 0x4014;

impl Bus for Nes2c02 {
    fn bounds(&self) -> (u16, u16) {
        unimplemented!()
    }
    fn set(&mut self, address: u16, value: u8) {
        if address < PPU_ORIGIN {
            // nes base ram
        } else if address < PPU_END {
            println!("writing to ppu: 0x{address:x} = 0x{value:x}");
            match address & 0b111 {
                PPU_CONTROLLER => {
                    // controller
                    if self.temp.is_warm() {
                        self.control.0 = value;
                    }
                    // todo: it is possible for this to generate an instant NMI, if other conditions are met
                    // add this for maximum glitchiness
                    self.latch.write(value);
                }
                PPU_MASK => {
                    // mask
                    self.latch.write(value);
                    self.mask = MaskRegister(value);
                }
                PPU_STATUS => {
                    // status is readonly, ignore write
                    self.latch.write(value);
                }
                PPU_OAM_ADDRESS => {
                    // oam address
                    self.latch.write(value);
                    self.oam_address = value;
                }
                PPU_OAM_DATA => {
                    // oam data
                    self.latch.write(value);
                    self.oam_data = value;
                }
                PPU_SCROLL => {
                    // scroll
                    self.latch.write(value);
                    self.scroll = value;
                }
                PPU_ADDRESS => {
                    // address
                    self.latch.write(value);
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
                    self.latch.write(value);
                    // data not writable
                }
                // todo: this is unreachable in current configuration
                // PPU_OAM_DMA => {
                //     // oam dma
                //     self.latch.write(value);
                //     self.oam_dma = value;
                // }
                _ => {
                    unreachable!()
                }
            }
        } else {
            // other addresses handled by cartridge
            unimplemented!()
        }
    }
    fn get(&mut self, address: u16) -> u8 {
        if address < PPU_ORIGIN {
            unimplemented!()
        } else if address < PPU_END {
            println!("reading from ppu: 0x{address:x}");
            match address {
                PPU_CONTROLLER => {
                    // controller is writeonly, return latch value
                    self.latch.read()
                }
                PPU_MASK => {
                    // mask is writeonly, return latch
                    self.latch.read()
                }
                PPU_STATUS => {
                    // status is only three bits, low bits from latch are read
                    let value = 
                    (self.status.0 & 0b1110_0000) | (self.latch.read() & 0b0001_1111);
                    self.latch.write(value);
                    value
                }
                PPU_OAM_ADDRESS => {
                    // oam address is writeonly, return latch
                    self.latch.read()
                }
                PPU_OAM_DATA => {
                    // oam data
                    todo!()
                }
                PPU_SCROLL => {
                    // scroll
                    todo!()
                }
                PPU_ADDRESS => {
                    // address
                    match self.address {
                        AddressRegisterLatch::Unset => {
                            todo!()
                        }
                        AddressRegisterLatch::First(hi) => {
                            todo!()
                        }
                        AddressRegisterLatch::Second(_, _) => {
                            todo!()
                        }
                    }
                }
                PPU_DATA => {
                    // data not writable
                    todo!()
                }
                PPU_OAM_DMA => {
                    // oam dma
                    todo!()
                }
                _ => {
                    // todo: implement appropriate mirroring
                    unimplemented!()
                }
            }
        } else {
            // other addresses handled by cartridge
            unimplemented!()
        }
    }
}

impl Default for Nes2c02 {
    fn default() -> Nes2c02 {
        Nes2c02 {
            latch: DecayingLatch::Decaying(0),
            control: ControlRegister(0),
            temp: Temperature::Cold(0),
            v_blank: false,
            mask: MaskRegister(0),
            status: StatusRegister(0),
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


#[repr(transparent)]
#[derive(Clone, Copy, Default)]
struct StatusRegister(u8);

impl StatusRegister {
    // todo: implement this...
}


#[repr(transparent)]
#[derive(Clone, Copy, Default)]
struct MaskRegister(u8);
impl MaskRegister {
    /// draw screen in greyscale
    fn greyscale(self) -> bool {
        self.0 & 0b1 > 0
    }
    /// display bg in leftmost 8 pixels
    fn margin_bg(self) -> bool {
        self.0 & 0b10 > 0
    }
    /// display fg in leftmost 8 pixels
    fn margin_fg(self) -> bool {
        self.0 & 0b100 > 0
    }
    /// display bg
    fn display_bg(self) -> bool {
        self.0 & 0b1000 > 0
    }
    /// display fg
    fn display_fg(self) -> bool {
        self.0 & 0b1_0000 > 0
    }
    /// ephasize r channel (green in PAL)
    fn emphasize_r(self) -> bool {
        self.0 & 0b10_0000 > 0
    }
    /// ephasize g channel (red in PAL)
    fn emphasize_g(self) -> bool {
        self.0 & 0b100_0000 > 0
    }
    /// ephasize b channel
    fn emphasize_b(self) -> bool {
        self.0 & 0b1000_0000 > 0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Default)]
struct ControlRegister(u8);

impl ControlRegister {
    fn base_nametable_address(self) -> u16 {
        match self.0 & 0b11 {
            0b00 => 0x2000,
            0b01 => 0x2400,
            0b10 => 0x2800,
            0b11 => 0x2c00,
            _ => unreachable!(),
        }
    }
    fn vram_address_increment(self) -> u16 {
        match self.0 & 0b100 {
            0b000 => 1,
            0b100 => 32,
            _ => unreachable!(),
        }
    }
    /// for 8x8 entries, ignored for 8/16
    fn sprite_pattern_table_address(self) -> u16 {
        (self.0 & 0b1000) as u16
    }
    fn background_pattern_table_address(self) -> u16 {
        match self.0 & 0b1_0000 {
            0b0_0000 => 0b0000,
            0b1_0000 => 0b1000,
            _ => unreachable!(),
        }
    }
    fn sprite_size(self) -> SpriteSize {
        use SpriteSize::*;
        match self.0 & 0b10_0000 {
            0b00_0000 => EightByEight,
            0b10_0000 => EightBySixteen,
            _ => unreachable!(),
        }
    }
    /// whether to read background data or write it on EXT pins
    /// actual NES hardare has the EXT pins grounded, so
    /// we will omit implementing this for now
    fn mode(self) -> ReadWriteMode {
        use ReadWriteMode::*;
        match self.0 & 0b100_0000 {
            0b000_0000 => Read,
            0b100_0000 => Write,
            _ => unreachable!(),
        }
    }
    /// whether to generate an NMI on vblank
    fn interrupt(self) -> bool {
        self.0 & 0b1000_0000 > 0
    }
}

enum SpriteSize {
    EightByEight,
    EightBySixteen,
}

enum ReadWriteMode {
    Read,
    Write,
}
