use std::collections::HashMap;

#[cfg(test)]
mod test;

mod opcodes;

pub mod prelude {
    // todo: select useful items to include in prelude
    pub use crate::*;
}

pub use opcodes::jumptable::opcode_jumptable;

/// allows a function to be called by an instance of the NES at each tick
pub trait NesPeripheral {
    fn tick(&mut self, nes: &mut Nes);
}

/// an instance of an NES machine
#[derive(Default)]
pub struct Nes {
    cpu: Nes2a03,
    ppu: Nes2c02,
    ram: NesRam,
    apu: Nes2a03Audio,
    cartridge: Option<NesCart>,
    gamepads: [Option<NesGamepad>; 8],
    peripherals: Option<Vec<Box<dyn NesPeripheral>>>,
}

impl Nes {
    pub fn extract_memory(&self, address: u16) -> u8 {
        self.ram.get(address)
    }
    pub fn with_initial_memory(mut self, address: u16, memory: &[u8]) -> Nes {
        self.ram.set_region(address, memory);
        self
    }
    pub fn with_peripheral(mut self, p: Box<dyn NesPeripheral>) -> Nes {
        self.add_peripheral(p);
        self
    }
    pub fn add_peripheral(&mut self, p: Box<dyn NesPeripheral>) {
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
    fn get_address_from_mode(&mut self, mode: u8) -> u16 {
        self.ram.get_address_from_mode(mode, &mut self.cpu.registers)
    }
    /// steps into one instruction. returns the number of cycles consumed
    pub fn step(&mut self) -> usize {
        println!("stepping");
        let opcode = self.next_byte();
        // let mut cycles = 0;
        let instruction = unsafe {
            // SAFETY: this is safe because we generate the jumptable
            // with 256 entries, which covers all possible u8 indexes
            opcode_jumptable.get_unchecked(opcode as usize)
        };
        instruction.run(self);
        // todo: with the way we're doing this we can remove the cycles
        // from the instruction fn signature argument list
        // and not have that function return any values
        
        if let Some(mut peripherals) = self.peripherals.take() {
            for p in peripherals.iter_mut() {
                p.tick(self);
            }
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
    pub fn next_byte(&mut self) -> u8 {
        let value = self.ram.get(self.cpu.registers.pc);
        self.cpu.registers.pc += 1;
        value
    }
    pub fn insert_cartridge(&mut self, cart: NesCart) {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct NesCart;
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
            unimplemented!()
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
    pub fn status_zero(&self) -> bool {
        self.p & FLAG_ZERO == FLAG_ZERO
    }
    pub fn status_negative(&self) -> bool {
        self.p & FLAG_NEGATIVE == FLAG_NEGATIVE
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
        self.pc = value;
        self
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
    pub fn get_carry(&self) -> u8{
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
}

/// todo: construct this more carefully
impl Default for NesRam {
    fn default() -> Self {
        NesRam {
            inner: [0u8; 2048],
            rom: CartridgeRom::default(),
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
                self.get(registers.pc) as u16
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
                base.wrapping_add(base) as u16
            }
            6 => {
                // Relative
                unimplemented!()
            }
            7 => {
                // Absolute
                unimplemented!()
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
                unimplemented!()
            }
            12 => {
                // IndirectIndexed
                unimplemented!()
            }
            _ => {
                unimplemented!()
            }
        }
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
