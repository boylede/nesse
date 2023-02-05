use std::fmt::Write;
use std::io::Read;

#[cfg(test)]
mod test;

#[cfg(feature = "delta")]
mod emulator_state;

pub mod cartridge;
pub mod cpu;
pub mod mapper;
mod opcodes;
pub mod peripherals;
pub mod ppu;

pub use opcodes::opcode_debug::OPCODE_NAMES;

pub mod prelude {
    // todo: select useful items to include in prelude
    pub use crate::cartridge::NesCart;
    pub use crate::cpu::{NesRegisters, RegisterAccess};
    pub use crate::peripherals::NesPeripheral;
    pub use crate::ppu::Nes2c02;
    pub use crate::*;
}

pub use opcodes::jumptable::OPCODE_JUMPTABLE;

use crate::cartridge::NesCart;
use crate::cpu::Nes2a03;
use crate::cpu::{NesRegisters, RegisterAccess};
use crate::ppu::Nes2c02;
use peripherals::NesPeripheral;

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
    pub cpu: Nes2a03,
    pub ppu: Nes2c02,
    pub ram: NesRam,
    pub apu: Nes2a03Audio,
    pub cartridge: Option<NesCart>,
    pub gamepads: [Option<NesGamepad>; 8],
    // todo: switch to enum_dispatch
    pub peripherals: Option<Vec<&'a mut dyn NesPeripheral>>,
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
    assert_eq!(nes, 4592);
}

impl<'a> Nes<'a> {
    /// a single tick of the master clock
    /// ticks the cpu every 1 ticks and ticks the cpu every 12
    /// also increments a frame counter and calls Nes::on_frame() every 29780 cpu ticks.
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
    /// sets the cpu to running and runs until stopped by some external force
    pub fn master_clock_drive(&mut self) {
        self.cpu.running = true;
        while self.cpu.running {
            self.master_tick();
        }
    }
    /// calls on_vblank on every peripheral
    pub fn on_frame(&mut self) {
        if let Some(mut peripherals) = self.peripherals.take() {
            for p in peripherals.iter_mut() {
                p.on_vblank(self);
            }
            self.peripherals.replace(peripherals);
        }
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
    pub fn extract_memory(&mut self, address: u16) -> u8 {
        self.get(address)
    }
    pub fn extract_memory_region(&mut self, address: u16, size: u16) -> Vec<u8> {
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
    pub fn with_peripheral(mut self, p: &'a mut dyn NesPeripheral) -> Nes<'a> {
        self.add_peripheral(p);
        self
    }
    pub fn set_pc(&mut self, value: u16) {
        self.cpu.registers.pc = value;
    }
    pub fn add_peripheral(&mut self, p: &'a mut dyn NesPeripheral) {
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
    pub fn dump_stack(&mut self) -> String {
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
                self.get_short(value)
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
                (hi << 8) | lo
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
    /// steps into one instruction.
    pub fn step(&mut self) {
        if self.cpu.next_tick <= self.cpu.cycles {
            if let Some(mut peripherals) = self.peripherals.take() {
                for p in peripherals.iter_mut() {
                    p.tick(self);
                }
                self.peripherals.replace(peripherals);
            }
            let opcode = self.peek_pc();
            self.cpu.registers.pc += 1;
            let instruction = unsafe {
                // SAFETY: this is safe because we generate the jumptable
                // with 256 entries, which covers all possible u8 indexes
                OPCODE_JUMPTABLE.get_unchecked(opcode as usize)
            };
            let cycles_spent = instruction.run(self);
            self.cpu.next_tick = self.cpu.cycles + cycles_spent as u64;
        } else {
            self.cpu.cycles += 1;
        }
    }
    /// returns a string with the registers
    pub fn display_registers(&self) -> String {
        format!("{:?}", self.cpu.registers)
    }
    /// returns the value at memory\[pc++\]
    pub fn peek_pc(&mut self) -> u8 {
        self.get(self.cpu.registers.pc)
    }
    /// inserts the cartridge
    pub fn insert_cartridge(&mut self, cart: NesCart) {
        if self.cartridge.is_none() {
            self.cartridge = Some(cart);
        } else {
            //todo: do we need to unload the existing cart before discarding?
            unimplemented!()
        }
    }
}

impl<'a> Bus for Nes<'a> {
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
    fn get(&mut self, address: u16) -> u8 {
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
            if let Some(ref mut cart) = self.cartridge {
                cart.get(address)
            } else {
                0
            }
        }
    }
}

pub trait Bus {
    fn bounds(&self) -> (u16, u16);
    fn bounds_check(&self, address: u16) -> bool {
        let bounds = self.bounds();
        address >= bounds.0 && address < bounds.1
    }
    fn set(&mut self, address: u16, value: u8);
    fn get(&mut self, address: u16) -> u8;
    fn set_region(&mut self, address: u16, bytes: &[u8]) {
        for (offset, value) in bytes.iter().enumerate() {
            self.set(address + offset as u16, *value);
        }
    }
    fn get_region(&mut self, address: u16, size: u16) -> Vec<u8> {
        let mut v = Vec::with_capacity(size as usize);
        for i in 0..size {
            v.push(self.get(i + address));
        }
        v
    }
    fn get_short(&mut self, address: u16) -> u16 {
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

impl Bus for NesRam {
    fn bounds(&self) -> (u16, u16) {
        (0, 0x2000)
    }
    fn set(&mut self, address: u16, value: u8) {
        if self.bounds_check(address) {
            let mirror = address & 0x7ff;
            self.inner[mirror as usize] = value;
        }
    }
    fn get(&mut self, address: u16) -> u8 {
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
    registers: [u8; 0x18],
}

impl Bus for Nes2a03Audio {
    fn bounds(&self) -> (u16, u16) {
        (0x4000, 0x401F)
    }
    fn set(&mut self, address: u16, value: u8) {
        let offset = address.wrapping_sub(0x4000);
        // println!("check: {} vs {}", address, offset);
        if offset < 0x18 {
            self.registers[offset as usize] = value;
        } else {
            panic!(
                "attempted to set invalid memory range in apu. {:04X} = {:02X}",
                address, value
            );
        }
    }
    fn get(&mut self, address: u16) -> u8 {
        let offset = address.wrapping_sub(0x4000);
        // println!("check: {} vs {}", address, offset);
        if offset < 0x18 {
            self.registers[offset as usize]
        } else {
            panic!(
                "attempted to get invalid memory range in apu. {:04X}",
                address
            );
        }
    }
}

#[derive(Default)]
pub struct NesGamepad;
