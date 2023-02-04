use std::collections::HashMap;
use std::fmt::Write;
use std::io::Read;

#[cfg(test)]
mod test;

#[cfg(feature = "delta")]
mod emulator_state;
mod opcodes;
pub mod peripherals;

pub use opcodes::opcode_debug::opcode_names;

pub mod prelude {
    // todo: select useful items to include in prelude
    pub use crate::*;
}

pub use opcodes::jumptable::OPCODE_JUMPTABLE;

// the value loaded into pc is stored in this location
const INITIAL_PC_LOCATION: u16 = 0xfffc;
// A value added to the SP on every stack operation
pub const STACK_OFFSET: u16 = 1 << 8;
/// The value of the stack pointer on reset
pub const STACK_INITIAL: u8 = 0xFD;
/// the value of the status register on reset
const STATUS_INITIAL: u8 = 0b00100100;

/// an instance of an NES machine
#[derive(Default)]
pub struct Nes<'a> {
    cpu: Nes2a03,
    ppu: Nes2c02,
    ram: NesRam,
    apu: Nes2a03Audio,
    cartridge: Option<NesCart>,
    gamepads: [Option<NesGamepad>; 8],
    // todo: switch to enum_dispatch
    peripherals: Option<Vec<&'a mut NesPeripheral>>,
}

#[test]
fn nes_size() {
    let nes = std::mem::size_of::<Nes>();
    // let rw = std::mem::size_of::<RegisterWrite>();
    // let gw = std::mem::size_of::<GlobalEvent>();
    // let d = std::mem::size_of::<DeltaEvent>();
    // assert_eq!(mw, 4 );
    // assert_eq!(rw, 6 );
    // assert_eq!(gw, 1 );
    assert_eq!(nes, 16);
}

impl<'a> Nes<'a> {
    /// a single tick of the master clock
    pub fn master_tick(&mut self) {
        self.ppu.clock_counter += 1;
        if self.ppu.clock_counter >= 4 {
            self.ppu.tick(&mut self.cartridge);
            self.ppu.clock_counter -= 4;
        }
        self.cpu.clock_counter += 1;
        if self.cpu.clock_counter >= 12 {
            self.step();
            self.cpu.clock_counter -= 12;
            self.ppu.frame_clock += 1;
            if self.ppu.frame_clock >= 29780 {
                // note: alternate +1 on odd frames
                self.ppu.frame_clock -= 29780;
                self.on_frame();
            }
        }
    }
    pub fn master_clock_drive(&mut self) {
        self.cpu.running = true;
        while self.cpu.running {
            self.master_tick();
        }
    }
    pub fn on_frame(&mut self) {
        if let Some(mut peripherals) = self.peripherals.take() {
            for p in peripherals.iter_mut() {
                p.on_vblank(self);
            }
            self.peripherals.replace(peripherals);
        }
    }
    pub fn load_rom(&mut self, rom: &[u8]) {
        NesCart::from_slice(rom);
        unimplemented!()
    }
    pub fn init(&mut self) {
        self.cpu.registers.reset();
        self.cpu.cycles = 7; // todo: model startup
        let initial_pc = self.get_short(INITIAL_PC_LOCATION);
        // println!("setting pc to {:x}", initial_pc);
        self.set_pc(initial_pc);
        if let Some(mut peripherals) = self.peripherals.take() {
            for p in peripherals.iter_mut() {
                p.init(self);
            }
            self.peripherals.replace(peripherals);
        }
    }
    pub fn cleanup(&mut self) {
        if let Some(mut peripherals) = self.peripherals.take() {
            for p in peripherals.iter_mut() {
                p.cleanup(self);
            }
            self.peripherals.replace(peripherals);
        }
    }
    pub fn extract_memory(&self, address: u16) -> u8 {
        self.get(address)
    }
    pub fn extract_memory_region(&self, address: u16, size: u16) -> Vec<u8> {
        let mut v = Vec::with_capacity(size as usize);
        for i in 0..size {
            v.push(self.get(i + address));
        }
        v
    }
    pub fn with_initial_memory(mut self, address: u16, memory: &[u8]) -> Nes<'a> {
        self.set_region(address, memory);
        self
    }
    pub fn with_peripheral(mut self, p: &'a mut NesPeripheral) -> Nes<'a> {
        self.add_peripheral(p);
        self
    }
    pub fn set_pc(&mut self, value: u16) {
        self.cpu.registers.pc = value;
    }
    pub fn add_peripheral(&mut self, p: &'a mut NesPeripheral) {
        if let Some(ref mut v) = self.peripherals {
            v.push(p);
        } else {
            self.peripherals = Some(vec![p]);
        }
    }
    /// debug function to insert arbitrary bytes at current PC in memory, overwriting anything already present
    pub fn inject_operation(&mut self, op: &str) {
        op.split(' ')
            .map(|st| u8::from_str_radix(st, 16).unwrap_or(0))
            .enumerate()
            .for_each(|(i, op)| {
                let index = self.cpu.registers.pc + i as u16;
                self.set(index, op);
            });
    }
    pub fn dump_registers(&self) -> NesRegisters {
        self.cpu.registers.clone()
    }
    pub fn inject_memory_value(&mut self, address: u16, value: u8) {
        self.set(address, value);
    }
    pub fn inject_registers(&mut self, regs: NesRegisters) {
        self.cpu.registers = regs;
    }
    pub fn dump_stack(&self) -> String {
        let sp = self.cpu.registers.sp as u16 + STACK_OFFSET;
        let mut depth = ((self.cpu.registers.sp ^ 0xff) >> 1) as u16;
        let mut stack = format! {"{}: ", depth};
        while depth > 0 {
            let value = self.get(sp + depth);
            write!(stack, "{:x} / ", value).unwrap();
            depth -= 1;
        }
        stack
    }
    fn get_address_from_mode(&mut self, mode: u8) -> u16 {
        match mode {
            0 => {
                // Implicit
                panic!("Should not request a get with implicit address mode.");
            }
            1 => {
                // Accumulator
                panic!("Should not request a get with accumulator address mode.");
            }
            2 => {
                // Immediate
                let value = self.cpu.registers.pc;
                self.cpu.registers.pc += 1;
                value
            }
            3 => {
                // ZeroPage
                let value = self.get(self.cpu.registers.pc) as u16;
                self.cpu.registers.pc += 1;
                value
            }
            4 => {
                // ZeroPageX
                let base = self.get(self.cpu.registers.pc);
                self.cpu.registers.pc += 1;
                // todo: is this behavior correct? will wrap around zero page
                base.wrapping_add(self.cpu.registers.x) as u16
            }
            5 => {
                // ZeroPageY
                let base = self.get(self.cpu.registers.pc);
                self.cpu.registers.pc += 1;
                // todo: is this wrap intended before the cast to u16 or after
                base.wrapping_add(self.cpu.registers.y) as u16
            }
            6 => {
                // Relative
                let base = self.get(self.cpu.registers.pc);
                self.cpu.registers.pc += 1;
                base as u16
            }
            7 => {
                // Absolute
                let value = self.cpu.registers.pc;
                self.cpu.registers.pc += 2; // skips two bytes since pointers are a two byte value
                let deref = self.get_short(value);
                deref
            }
            8 => {
                // AbsoluteX
                let address = self.get_short(self.cpu.registers.pc);
                self.cpu.registers.pc += 2;
                address.wrapping_add(self.cpu.registers.x as u16)
            }
            9 => {
                // AbsoluteY
                let address = self.get_short(self.cpu.registers.pc);
                self.cpu.registers.pc += 2;
                address.wrapping_add(self.cpu.registers.y as u16)
            }
            10 => {
                // Indirect
                let address = self.get_short(self.cpu.registers.pc);
                self.cpu.registers.pc += 2;
                let address_lo = (address & 0xff) as u8;
                let address_hi = address & 0xff00;
                let lo = self.get(address) as u16;
                let hi_address = address_lo.wrapping_add(1) as u16 | address_hi;
                let hi = self.get(hi_address) as u16;
                (hi << 8) | lo
            }
            11 => {
                // IndexedIndirect (x)
                // The address of the table is taken from the instruction and the X register added to it (with zero page wrap around) to give the location of the least significant byte of the target address.
                let table = self.get(self.cpu.registers.pc);
                self.cpu.registers.pc += 1;
                let base = table.wrapping_add(self.cpu.registers.x);
                let lo = self.get(base as u16) as u16;
                let hi = self.get(base.wrapping_add(1) as u16) as u16;
                let value = (hi << 8) | lo;
                value
            }
            12 => {
                // IndirectIndexed (y)
                // let immediate = nes.get(pc + 1);
                // let lo = nes.get(immediate as u16) as u16;
                // let hi = nes.get(immediate.wrapping_add(1) as u16) as u16; // wraps around zero page
                // let short = hi << 8 | lo;
                // let address = short.wrapping_add(regs.y as u16);
                // let value = nes.get(address);
                let immediate = self.get(self.cpu.registers.pc);
                self.cpu.registers.pc += 1;
                let lo = self.get(immediate as u16) as u16;
                let hi = self.get(immediate.wrapping_add(1) as u16) as u16;
                let short = hi << 8 | lo;
                short.wrapping_add(self.cpu.registers.y as u16)
                // (hi as u16) << 8 | (lo as u16).wrapping_add(self.cpu.registers.y as u16)
            }
            _ => {
                unimplemented!()
            }
        }
    }
    pub fn stack_push(&mut self, value: u8) {
        let mut sp = self.cpu.registers.sp;
        self.set(sp as u16 + STACK_OFFSET, value);
        sp = sp.wrapping_sub(1);
        self.cpu.registers.sp = sp;
    }
    pub fn stack_pop(&mut self) -> u8 {
        let mut sp = self.cpu.registers.sp;
        sp = sp.wrapping_add(1);
        let value = self.get(sp as u16 + STACK_OFFSET);
        self.cpu.registers.sp = sp;
        value
    }
    pub fn stack_push_short(&mut self, value: u16) {
        let lo = (value & 0xff) as u8;
        let hi = ((value >> 8) & 0xff) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
    }
    pub fn stack_pop_short(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        (hi << 8) | lo
    }
    /// steps into one instruction. returns the number of cycles consumed
    pub fn step(&mut self) -> u64 {
        // println!("stepping");
        if let Some(mut peripherals) = self.peripherals.take() {
            for p in peripherals.iter_mut() {
                p.tick(self);
            }
            self.peripherals.replace(peripherals);
        }
        let opcode = self.peek_pc();
        self.cpu.registers.pc += 1;
        // let mut cycles = 0;
        let instruction = unsafe {
            // SAFETY: this is safe because we generate the jumptable
            // with 256 entries, which covers all possible u8 indexes
            OPCODE_JUMPTABLE.get_unchecked(opcode as usize)
        };
        instruction.run(self);
        // todo: with the way we're doing this we can remove the cycles
        // from the instruction fn signature argument list
        // and not have that function return any values
        self.cpu.cycles += instruction.cycles as u64;
        instruction.cycles as u64
    }
    pub fn run_until_nop(&mut self) -> u64 {
        let mut last = self.step();
        let mut total = last;
        self.cpu.running = true;
        while self.cpu.running {
            last = self.step();
            total += last;
        }
        total
    }
    pub fn display_registers(&self) -> String {
        format!("{:?}", self.cpu.registers)
    }
    /// returns the value at memory\[pc++\]
    pub fn peek_pc(&mut self) -> u8 {
        self.get(self.cpu.registers.pc)
    }
    pub fn insert_cartridge(&mut self, cart: NesCart) {
        if let None = self.cartridge {
            self.cartridge = Some(cart);
        } else {
            //todo: do we need to unload the existing cart before discarding?
            unimplemented!()
        }
    }
}

impl<'a> AddressableMemory for Nes<'a> {
    fn bounds(&self) -> (u16, u16) {
        (0, 0xffff)
    }
    fn set(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            // nes base ram
            self.ram.set(address, value);
        } else if address < 0x4000 {
            // ppu access
            self.ppu.set(address, value);
        } else if address < 0x4020 {
            // apu registers
            self.apu.set(address, value);
        } else {
            // other addresses handled by cartridge
            if let Some(cart) = &mut self.cartridge {
                cart.set(address, value);
            }
        }
    }
    fn get(&self, address: u16) -> u8 {
        if address < 0x2000 {
            // nes base ram
            self.ram.get(address)
        } else if address < 0x4000 {
            // ppu access
            self.ppu.get(address)
        } else if address < 0x4020 {
            // apu registers
            self.apu.get(address)
        } else {
            // other addresses handled by cartridge
            if let Some(cart) = &self.cartridge {
                cart.get(address)
            } else {
                0
            }
        }
    }
}

impl<'a> RegisterAccess for Nes<'a> {
    fn get_a(&self) -> u8 {
        self.cpu.registers.a
    }
    fn get_x(&self) -> u8 {
        self.cpu.registers.x
    }
    fn get_y(&self) -> u8 {
        self.cpu.registers.y
    }
    fn get_p(&self) -> u8 {
        self.cpu.registers.p
    }
    fn get_sp(&self) -> u8 {
        self.cpu.registers.sp
    }
    fn get_pc(&self) -> u16 {
        self.cpu.registers.pc
    }

    fn set_a(&mut self, value: u8) -> u8 {
        let old = self.cpu.registers.a;
        self.cpu.registers.a = value;
        old
    }
    fn set_x(&mut self, value: u8) -> u8 {
        let old = self.cpu.registers.x;
        self.cpu.registers.x = value;
        old
    }
    fn set_y(&mut self, value: u8) -> u8 {
        let old = self.cpu.registers.y;
        self.cpu.registers.y = value;
        old
    }
    fn set_p(&mut self, value: u8) -> u8 {
        let old = self.cpu.registers.p;
        self.cpu.registers.p = value;
        old
    }
    fn set_sp(&mut self, value: u8) -> u8 {
        let old = self.cpu.registers.sp;
        self.cpu.registers.sp = value;
        old
    }
    fn set_pc(&mut self, value: u16) -> u16 {
        let old = self.cpu.registers.pc;
        self.cpu.registers.pc = value;
        old
    }
}

pub struct NesCart {
    pub header: NesCartHeader,
    trainer: Option<Vec<u8>>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,
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
        let reserved = header[9];
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
        bytes.read_exact(&mut prg_rom).unwrap();
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

        // unimplemented!();
        Some(NesCart {
            header,
            trainer,
            prg_rom,
            chr_rom,
            prg_ram,
        })
    }
    pub fn simple(start_addrss: u16, rom: &[u8]) -> NesCart {
        unimplemented!()
    }
}

impl AddressableMemory for NesCart {
    fn bounds(&self) -> (u16, u16) {
        (0x4020, 0xffff)
    }
    fn set(&mut self, address: u16, value: u8) {
        // todo: bounds checks
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
    fn get(&self, address: u16) -> u8 {
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

/// NES cpu instance
#[derive(Default, Clone)]
pub struct Nes2a03 {
    running: bool,
    cycles: u64,
    clock_counter: u8,
    registers: NesRegisters,
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct NesRegisters {
    /// program counter
    pub pc: u16,
    /// stack pointer
    pub sp: u8,
    /// accumulator
    pub a: u8,
    /// index x
    pub x: u8,
    /// index y
    pub y: u8,
    /// processor status
    pub p: u8,
    // debug counter for stack pushes
    // debug_stack_depth: u16,
}

const FLAG_CARRY: u8 = 1 << 0;
const FLAG_ZERO: u8 = 1 << 1;
const FLAG_INTERRUPT: u8 = 1 << 2;
const FLAG_DECIMAL: u8 = 1 << 3;
const FLAG_BL: u8 = 1 << 4; // B LOW BIT
const FLAG_BH: u8 = 1 << 5; // B HIGH BIT
const FLAG_OVERFLOW: u8 = 1 << 6;
const FLAG_NEGATIVE: u8 = 1 << 7;

impl NesRegisters {
    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = STACK_INITIAL;
        self.p = STATUS_INITIAL;
        // self.pc = todo: figure out how i want to handle multiple options here
    }
    pub fn get_status_stack(&self) -> u8 {
        let mut value = self.p;
        value |= FLAG_BL | FLAG_BH;
        value
    }
    pub fn set_status_stack(&mut self, value: u8) {
        let current_bh = self.p & (FLAG_BH | FLAG_BL);
        let new_status = (value & !(FLAG_BH | FLAG_BL)) | current_bh;
        // println!("setting status {:08b}", new_status);
        self.p = new_status;
    }
    pub fn status_zero(&self) -> bool {
        self.p & FLAG_ZERO == FLAG_ZERO
    }
    pub fn status_negative(&self) -> bool {
        self.p & FLAG_NEGATIVE == FLAG_NEGATIVE
    }
    pub fn status_carry(&self) -> bool {
        self.p & FLAG_CARRY == FLAG_CARRY
    }
    pub fn status_overflow(&self) -> bool {
        self.p & FLAG_OVERFLOW == FLAG_OVERFLOW
    }
    pub fn with_a(mut self, value: u8) -> Self {
        self.a = value;
        self
    }
    pub fn with_x(mut self, value: u8) -> Self {
        self.x = value;
        self
    }
    pub fn with_flags_from(mut self, value: u8) -> Self {
        self.set_flags(value);
        self
    }
    pub fn with_pc(mut self, value: u16) -> Self {
        self.set_pc(value);
        self
    }
    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }
    pub fn get_pc(&self) -> u16 {
        self.pc
    }
    pub fn set_flags(&mut self, value: u8) {
        if value == 0 {
            self.p |= FLAG_ZERO;
        } else {
            self.p &= !FLAG_ZERO;
        }
        if (value as i8) < 0 {
            self.p |= FLAG_NEGATIVE;
        } else {
            self.p &= !FLAG_NEGATIVE;
        }
    }
    pub fn set_overflow_from(&mut self, test: u8) {
        // todo: can we remove conditional here
        if (test & FLAG_OVERFLOW) > 0 {
            self.p |= FLAG_OVERFLOW;
        } else {
            self.p &= !FLAG_OVERFLOW;
        }
    }
    pub fn set_negative_from(&mut self, test: u8) {
        // todo: can we remove conditional here
        if (test & FLAG_NEGATIVE) > 0 {
            self.p |= FLAG_NEGATIVE;
        } else {
            self.p &= !FLAG_NEGATIVE;
        }
    }
    pub fn set_zero_from(&mut self, test: u8) {
        if test == 0 {
            self.p |= FLAG_ZERO;
        } else {
            self.p &= !FLAG_ZERO;
        }
    }
    pub fn set_carry(&mut self) {
        self.p |= FLAG_CARRY;
    }
    pub fn clear_carry(&mut self) {
        self.p &= !FLAG_CARRY;
    }
    pub fn set_interrupt(&mut self) {
        self.p |= FLAG_INTERRUPT;
    }
    pub fn clear_interrupt(&mut self) {
        self.p &= !FLAG_INTERRUPT;
    }
    pub fn set_decimal(&mut self) {
        self.p |= FLAG_DECIMAL;
    }
    pub fn clear_decimal(&mut self) {
        self.p &= !FLAG_DECIMAL;
    }
    pub fn set_overflow(&mut self) {
        self.p |= FLAG_OVERFLOW;
    }
    pub fn clear_overflow(&mut self) {
        self.p &= !FLAG_OVERFLOW;
    }
    pub fn set_negative(&mut self) {
        self.p |= FLAG_NEGATIVE;
    }
    pub fn clear_negative(&mut self) {
        self.p &= !FLAG_NEGATIVE;
    }
    pub fn get_carry(&self) -> u8 {
        self.p & FLAG_CARRY
    }
    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }
    pub fn set_p(&mut self, value: u8) {
        self.p = value;
    }
}

/// nes ppu instance
pub struct Nes2c02 {
    controller: u8,
    mask: u8,
    status: u8,
    oam_address: u8,
    oam_data: u8,
    scroll: u8,
    address: u8,
    data: u8,
    oam_dma: u8,
    // 0, 1, 2, or 3 to indicate syncronization between cpu and ppu clocks
    timing: u8,
    clock_counter: u8,
    frame_clock: u16,
    pallete_table: [u8; 32],
    vram: [u8; 2048],
    oam: [u8; 256],
}

impl Nes2c02 {
    pub fn tick(&mut self, cart: &mut Option<NesCart>) {
        // tick
    }
}
impl AddressableMemory for Nes2c02 {
    fn bounds(&self) -> (u16, u16) {
        unimplemented!()
    }
    fn set(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            // nes base ram
        } else if address < 0x4020 {
            // other hardware
            println!("837: wanted to write {:02X} to address {:04X}", value, address);
            unimplemented!()
        } else {
            // other addresses handled by cartridge
            unimplemented!()
        }
    }
    fn get(&self, address: u16) -> u8 {
        if address < 0x2000 {
            unimplemented!()
        } else if address < 0x4020 {
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
            address: 0u8,
            data: 0u8,
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

pub trait RegisterAccess {
    fn get_a(&self) -> u8;
    fn get_x(&self) -> u8;
    fn get_y(&self) -> u8;
    fn get_p(&self) -> u8;
    fn get_sp(&self) -> u8;
    fn get_pc(&self) -> u16;

    /// sets "a" register, returns old value.
    fn set_a(&mut self, value: u8) -> u8;
    fn set_x(&mut self, value: u8) -> u8;
    fn set_y(&mut self, value: u8) -> u8;
    fn set_p(&mut self, value: u8) -> u8;
    fn set_sp(&mut self, value: u8) -> u8;
    fn set_pc(&mut self, value: u16) -> u16;

    // provided methods
    /// sets all registers to startup values, except PC which must be set separately
    fn reset(&mut self) {
        self.set_a(0);
        self.set_x(0);
        self.set_y(0);
        self.set_sp(STACK_INITIAL);
        self.set_p(STATUS_INITIAL);
    }
    fn status_zero(&self) -> bool {
        self.get_p() & FLAG_ZERO == FLAG_ZERO
    }
    fn status_negative(&self) -> bool {
        self.get_p() & FLAG_NEGATIVE == FLAG_NEGATIVE
    }
    fn status_carry(&self) -> bool {
        self.get_p() & FLAG_CARRY == FLAG_CARRY
    }
    fn set_flags_from(&mut self, value: u8) {
        self.set_zero_from(value);
        self.set_negative_from(value);
    }
    fn set_overflow_from(&mut self, test: u8) {
        // todo: can we remove conditional here
        if (test & FLAG_OVERFLOW) > 0 {
            self.set_p(self.get_p() | FLAG_OVERFLOW);
        } else {
            self.set_p(self.get_p() & !FLAG_OVERFLOW);
        }
    }
    fn set_negative_from(&mut self, test: u8) {
        // todo: can we remove conditional here
        if (test & FLAG_NEGATIVE) > 0 {
            self.set_negative();
        } else {
            self.clear_negative();
        }
    }
    fn set_zero_from(&mut self, test: u8) {
        if test == 0 {
            self.set_zero()
        } else {
            self.clear_zero()
        }
    }
    fn set_zero(&mut self) {
        self.set_p(self.get_p() | FLAG_ZERO);
    }
    fn clear_zero(&mut self) {
        self.set_p(self.get_p() & !FLAG_ZERO);
    }
    fn set_carry(&mut self) {
        self.set_p(self.get_p() | FLAG_CARRY);
    }
    fn clear_carry(&mut self) {
        self.set_p(self.get_p() & !FLAG_CARRY);
    }
    fn set_overflow(&mut self) {
        self.set_p(self.get_p() | FLAG_OVERFLOW);
    }
    fn clear_overflow(&mut self) {
        self.set_p(self.get_p() & !FLAG_OVERFLOW);
    }
    fn set_negative(&mut self) {
        self.set_p(self.get_p() | FLAG_NEGATIVE);
    }
    fn clear_negative(&mut self) {
        self.set_p(self.get_p() & !FLAG_NEGATIVE);
    }
    fn get_carry(&self) -> u8 {
        self.get_p() & FLAG_CARRY
    }
    fn increment_a(&mut self) {
        self.set_a(self.get_a().wrapping_add(1));
    }
    fn increment_pc(&mut self) {
        self.set_pc(self.get_pc().wrapping_add(1));
    }
    fn increment_x(&mut self) {
        self.set_x(self.get_x().wrapping_add(1));
    }
    fn increment_y(&mut self) {
        self.set_y(self.get_y().wrapping_add(1));
    }
}

pub trait AddressableMemory {
    fn bounds(&self) -> (u16, u16);
    fn bounds_check(&self, address: u16) -> bool {
        let bounds = self.bounds();
        address >= bounds.0 && address < bounds.1
    }
    fn set(&mut self, address: u16, value: u8);
    fn get(&self, address: u16) -> u8;
    fn set_region(&mut self, address: u16, bytes: &[u8]) {
        for (offset, value) in bytes.iter().enumerate() {
            self.set(address + offset as u16, *value);
        }
    }
    fn get_region(&self, address: u16, size: u16) -> Vec<u8> {
        let mut v = Vec::with_capacity(size as usize);
        for i in 0..size {
            v.push(self.get(i + address));
        }
        v
    }
    fn get_short(&self, address: u16) -> u16 {
        let low = self.get(address) as u16;
        let high = self.get(address + 1) as u16;
        high << 8 | low
    }
    fn set_short(&mut self, address: u16, value: u16) {
        let low = (value & 0xff) as u8;
        let high = ((value >> 8) & 0xff) as u8;
        self.set(address, low);
        self.set(address + 1, high);
    }
}

pub struct NesRam {
    inner: [u8; 2048],
}

/// todo: construct this more carefully
impl Default for NesRam {
    fn default() -> Self {
        NesRam { inner: [0u8; 2048] }
    }
}

impl AddressableMemory for NesRam {
    fn bounds(&self) -> (u16, u16) {
        (0, 0x2000)
    }
    fn set(&mut self, address: u16, value: u8) {
        if self.bounds_check(address) {
            let mirror = address & 0x7ff;
            self.inner[mirror as usize] = value;
        }
    }
    fn get(&self, address: u16) -> u8 {
        if self.bounds_check(address) {
            self.inner[address as usize]
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct Nes2a03Audio {
    /// apu register $4000,$4001,$4002,$4003,
    // pulse_1: u32,
    /// apu register $4004,$4005,$4006,$4007,
    // pulse_2: u32,
    /// apu register $4008,$4009,$400A,$400B,
    // triangle: u32,
    /// apu register $400C,$400D,$400E,$400F,
    // noise: u32,
    /// apu register $4010,$4011,$4012,$4013,
    // dmc: u32,
    /// apu register $4015
    // status: u8,
    /// apu register $4017
    /// also called frame counter in docs
    // frame_sequencer: u8,
    registers: [u8;0x18],
}

impl AddressableMemory for Nes2a03Audio {
    fn bounds(&self) -> (u16, u16) {
        (0x4000, 0x401F)
    }
    fn set(&mut self, address: u16, value: u8) {
        let offset = address.wrapping_sub(0x4000);
        // println!("check: {} vs {}", address, offset);
        if offset < 0x18 {
            self.registers[offset as usize] = value;
        } else {
            panic!("attempted to set invalid memory range in apu. {:04X} = {:02X}", address, value);
        }
    }
    fn get(&self, address: u16) -> u8 {
        let offset = address.wrapping_sub(0x4000);
        // println!("check: {} vs {}", address, offset);
        if offset < 0x18 {
            self.registers[offset as usize]
        } else {
            panic!("attempted to get invalid memory range in apu. {:04X}", address);
        }
    }
}

#[derive(Default)]
pub struct NesGamepad;
