use std::collections::HashMap;
use std::fmt::Write;

#[cfg(test)]
mod test;

mod opcodes;

pub mod prelude {
    // todo: select useful items to include in prelude
    pub use crate::*;
}

pub use opcodes::jumptable::OPCODE_JUMPTABLE;

// the value loaded into pc is stored in this location
const INITIAL_PC_LOCATION: u16 = 0xfffc;
// A value added to the SP on every stack operation
const STACK_OFFSET: u16 = 1 << 8;
/// The value of the stack pointer on reset
const STACK_INITIAL: u8 = 0xFD;

/// allows a function to be called by an instance of the NES at each tick
pub trait NesPeripheral {
    fn init(&mut self, nes: &mut Nes) {}
    fn tick(&mut self, nes: &mut Nes) {}
    fn cleanup(&mut self, nes: &mut Nes) {}
}

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

impl<'a> Nes<'a> {
    pub fn load_rom(&mut self, rom: &[u8]) {
        NesCart::from_slice(rom);
        unimplemented!()
    }
    pub fn init(&mut self) {
        self.cpu.registers.reset();
        self.set_pc(self.ram.get_short(INITIAL_PC_LOCATION));
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
        }
    }
    pub fn extract_memory(&self, address: u16) -> u8 {
        self.ram.get(address)
    }
    pub fn extract_memory_region(&self, address: u16, size: u16) -> Vec<u8> {
        let mut v = Vec::with_capacity(size as usize);
        for i in 0..size {
            v.push(self.ram.get(i + address));
        }
        v
    }
    pub fn with_initial_memory(mut self, address: u16, memory: &[u8]) -> Nes<'a> {
        self.ram.set_region(address, memory);
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
                self.ram.set(index, op);
            });
    }
    pub fn dump_registers(&self) -> NesRegisters {
        self.cpu.registers.clone()
    }
    pub fn inject_memory_value(&mut self, address: u16, value: u8) {
        self.ram.set(address, value);
    }
    pub fn inject_registers(&mut self, regs: NesRegisters) {
        self.cpu.registers = regs;
    }
    pub fn dump_stack(&self) -> String {
        let sp = self.cpu.registers.sp as u16 + STACK_OFFSET;
        let mut depth = self.ram.debug_stack_depth;
        let mut stack = format! {"{}: ", depth};
        while depth > 0 {
            let value = self.ram.get(sp + depth);
            write!(stack, "{:x} / ", value).unwrap();
            depth -= 1;
        }
        stack
    }
    fn get_address_from_mode(&mut self, mode: u8) -> u16 {
        self.ram
            .get_address_from_mode(mode, &mut self.cpu.registers)
    }
    /// steps into one instruction. returns the number of cycles consumed
    pub fn step(&mut self) -> usize {
        // println!("stepping");
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

        if let Some(mut peripherals) = self.peripherals.take() {
            for p in peripherals.iter_mut() {
                p.tick(self);
            }
            self.peripherals.replace(peripherals);
        }
        instruction.cycles as usize
    }
    pub fn run_until_nop(&mut self) -> usize {
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
        let value = self.ram.get(self.cpu.registers.pc);

        value
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

pub struct NesCart {
    pub header: NesCartHeader,
    trainer: Option<Vec<u8>>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>
}

pub struct NesCartHeader {
    mapper_id: u8,
    mirroring: u8,
    four_screen: bool,
    battery: bool,
}

impl NesCart {
    pub fn from_slice(bytes: &[u8]) -> Option<NesCart> {
        let buffer = bytes.to_vec();
        let header: Vec<u8> = buffer.iter().take(16).copied().collect();
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
            println!("unexpected values reserved area of rom header");
            return None;
        }
        println!("number of 16kB rom banks: {}", rom_count);
        println!("number of 8kB vrom banks: {}", vrom_count);
        // flags in control byte 0 (aka 1 in references)
        const FLAG_MIRRORING :u8 = 1 << 0;
        const FLAG_BBRAM :u8 = 1 << 1;
        const FLAG_TRAINER :u8 = 1 << 2;
        const FLAG_FOUR_SCREEN :u8 = 1 << 3;
        // flags in control byte 1 (aka 2 in references)
        const FLAG_RESERVED_0 :u8 = 1 << 0;
        const FLAG_RESERVED_1 :u8 = 1 << 1;
        const FLAG_RESERVED_2 :u8 = 1 << 2;
        const FLAG_RESERVED_3 :u8 = 1 << 3; // one if iNES2.0
        
        let mapper_id = {
            let upper = control_bytes[1] & 0xf0;
            let lower = control_bytes[0] & 0xf0;
            upper  | lower >> 4
        };

        
        let mirroring = control_bytes[0] & FLAG_MIRRORING;
        println!("mirroring mode: {}", mirroring);
        let battery = control_bytes[0] & FLAG_BBRAM > 0; // at 0x6000..0x7fff
        println!("has battery: {}", battery);
        let has_trainer = control_bytes[0] & FLAG_TRAINER > 0;
        println!("has trainer: {}", has_trainer);
        let four_screen = (control_bytes[0] & FLAG_FOUR_SCREEN) > 0;
        println!("uses four screen mirroring: {}", four_screen);
        println!("expects mapper {}", mapper_id);
        println!("number of 8kB ram banks: {}", ram_count);
        println!("other byte: {:x}", reserved);

        let unhandled_bits = FLAG_RESERVED_0 | FLAG_RESERVED_1 | FLAG_RESERVED_2 | FLAG_RESERVED_3;
        let unhandled = control_bytes[1] & unhandled_bits;

        if unhandled != 0 {
            println!("has unexpected items in control byte 2, may be different file version: {:x}", unhandled);
            return None;
        }
        let header = NesCartHeader {
            mapper_id,
            mirroring,
            four_screen,
            battery,
        };
        
        let trainer = if has_trainer {
            let t = buffer.iter().take(512).copied().collect();
            Some(t)
        } else {
            None
        };
        let prg_rom_size = rom_count as usize * 16 * 1024;
        let prg_rom: Vec<u8> = buffer.iter().take(prg_rom_size).copied().collect();
        println!("retreived {} bytes for prg_rom", prg_rom.len());
        let chr_rom_size = vrom_count as usize * 16 * 1024;
        let chr_rom: Vec<u8> = buffer.iter().take(chr_rom_size).copied().collect();
        println!("retreived {} bytes for chr_rom", chr_rom.len());


        let prg_ram = if battery {
            // todo: do we load in battery-backed ram from another file?
            unimplemented!()
        } else {
            Vec::with_capacity(8*1024*ram_count as usize)
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
#[derive(Default)]
pub struct CartridgeRom {
    // todo: a better way to do this so that we don't have to store the whole address space but still don't have a constant size
    inner: HashMap<u16, u8>,
}

impl CartridgeRom {
    pub fn from_vec(bytes: Vec<u8>) -> CartridgeRom {
        CartridgeRom {
            inner: bytes
                .iter()
                .enumerate()
                .map(|(i, v)| (i as u16, *v))
                .collect(),
        }
    }
    pub fn get(&self, index: u16) -> u8 {
        // todo: bounds checks
        // todo: hardware mapping redirect
        if index < 0x2000 {
            unimplemented!() // should be the cpu's ram
        } else if index < 0x4020 {
            // other hardware
            unimplemented!()
        } else if index < 0x6000 {
            // special depending on cartridge generation
            unimplemented!()
        } else if index < 0x8000 {
            // option ram, for e.g. zelda
            unimplemented!()
        } else {
            // cartridge rom
            self.inner.get(&index).copied().unwrap_or(0)
        }
    }
    pub fn get_short(&self, index: u16) -> u16 {
        let low = self.get(index) as u16;
        let high = self.get(index + 1) as u16;
        high << 8 | low
    }
}

/// NES cpu instance
#[derive(Default, Clone)]
pub struct Nes2a03 {
    running: bool,
    registers: NesRegisters,
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct NesRegisters {
    /// program counter
    pc: u16,
    /// stack pointer
    sp: u8,
    /// accumulator
    a: u8,
    /// index x
    x: u8,
    /// index y
    y: u8,
    /// processor status
    p: u8,
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
        // self.debug_stack_depth = 0;
        self.p = 0b100100;
        // self.pc = todo: figure out how i want to handle multiple options here
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
}

#[derive(Default)]
pub struct Nes2c02;

/// this will manage all memory accesses, including ones which do not go to the onboard ram chip
/// todo: may reorganize to make more sense
pub struct NesRam {
    inner: [u8; 2048],
    rom: CartridgeRom,
    debug_stack_depth: u16,
}

/// todo: construct this more carefully
impl Default for NesRam {
    fn default() -> Self {
        NesRam {
            inner: [0u8; 2048],
            rom: CartridgeRom::default(),
            debug_stack_depth: 0,
        }
    }
}

impl NesRam {
    pub fn set_region(&mut self, start: u16, bytes: &[u8]) {
        for (offset, value) in bytes.iter().enumerate() {
            self.set(start + offset as u16, *value);
        }
    }
    pub fn get_address_from_mode(&self, mode: u8, registers: &mut NesRegisters) -> u16 {
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
                let value = registers.pc;
                registers.pc += 1;
                value
            }
            3 => {
                // ZeroPage
                let value = self.get(registers.pc) as u16;
                registers.pc += 1;
                value
            }
            4 => {
                // ZeroPageX
                let base = self.get(registers.pc);
                registers.pc += 1;
                // todo: is this behavior correct? will wrap around zero page
                base.wrapping_add(registers.x) as u16
            }
            5 => {
                // ZeroPageY
                let base = self.get(registers.pc);
                registers.pc += 1;
                // todo: is this wrap intended before the cast to u16 or after
                base.wrapping_add(registers.y) as u16
            }
            6 => {
                // Relative
                let base = self.get(registers.pc);
                registers.pc += 1;
                base as u16
            }
            7 => {
                // Absolute
                let value = registers.pc;
                registers.pc += 2; // skips two bytes since pointers are a two byte value
                value
            }
            8 => {
                // AbsoluteX
                unimplemented!()
            }
            9 => {
                // AbsoluteY
                unimplemented!()
            }
            10 => {
                // Indirect
                unimplemented!()
            }
            11 => {
                // IndexedIndirect
                // The address of the table is taken from the instruction and the X register added to it (with zero page wrap around) to give the location of the least significant byte of the target address.
                let table = self.get(registers.pc);
                registers.pc += 1;
                let base = table.wrapping_add(registers.x) as u16;
                let lo = self.get(base) as u16;
                let hi = self.get(base.wrapping_add(1)) as u16;
                (hi << 8) | lo
            }
            12 => {
                // IndirectIndexed
                let base = self.get(registers.pc);
                registers.pc += 1;
                let lo = self.get(base as u16);
                let hi = self.get(base.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16).wrapping_add(registers.y as u16)
            }
            _ => {
                unimplemented!()
            }
        }
    }
    pub fn stack_push(&mut self, sp: &mut u8, value: u8) {
        // println!("pushing {:X} to stack", value);
        self.set(*sp as u16 + STACK_OFFSET, value);
        *sp = sp.wrapping_sub(1);
        self.debug_stack_depth += 1;
    }
    pub fn stack_pop(&mut self, sp: &mut u8) -> u8 {
        *sp = sp.wrapping_add(1);
        let value = self.get(*sp as u16 + STACK_OFFSET);
        self.debug_stack_depth -= 1;
        value
    }
    pub fn stack_push_short(&mut self, sp: &mut u8, value: u16) {
        self.stack_push(sp, (value & 0xff) as u8);
        self.stack_push(sp, ((value >> 8) & 0xff) as u8);
    }
    pub fn stack_pop_short(&mut self, sp: &mut u8) -> u16 {
        let high = self.stack_pop(sp) as u16;
        let low = self.stack_pop(sp) as u16;
        let value = (high << 8) | low;
        // println!("popped short {:x} from stack", value);
        value
    }
    pub fn load_cartridge(&mut self, cartridge: NesCart) {
        unimplemented!()
    }
    pub fn set(&mut self, index: u16, value: u8) {
        // todo: hardware mapping redirect
        if index < 0x2000 {
            self.inner[index as usize] = value;
        } else if index < 0x4020 {
            // other hardware
        } else if index < 0x6000 {
            // special depending on cartridge generation
        } else if index < 0x8000 {
            // option ram, for e.g. zelda
        } else {
            // cartridge rom
        }
    }
    pub fn get(&self, index: u16) -> u8 {
        // todo: bounds checks
        // todo: hardware mapping redirect
        if index < 0x2000 {
            self.inner[index as usize]
        } else if index < 0x4020 {
            // other hardware
            unimplemented!()
        } else if index < 0x6000 {
            // special depending on cartridge generation
            unimplemented!()
        } else if index < 0x8000 {
            // option ram, for e.g. zelda
            unimplemented!()
        } else {
            // cartridge rom
            unimplemented!()
        }
    }
    pub fn get_short(&self, index: u16) -> u16 {
        let low = self.get(index) as u16;
        let high = self.get(index + 1) as u16;
        high << 8 | low
    }
    pub fn set_short(&mut self, index: u16, value: u16) {
        let low = (value & 0xff) as u8;
        let high = ((value >> 8) & 0xff) as u8;
        self.set(index, low);
        self.set(index + 1, high);
    }
}

#[derive(Default)]
pub struct Nes2a03Audio;

#[derive(Default)]
pub struct NesGamepad;
