use crate::STACK_INITIAL;
use crate::STATUS_INITIAL;
/// NES cpu instance
#[derive(Default, Clone)]
pub struct Nes2a03 {
    /// if the cpu should tick
    pub running: bool,
    /// total number of cycles consumed
    pub cycles: u64,
    /// counter for when to process next instruction
    pub next_tick: u64,
    /// small counter for dividing the master clock ticks without using division
    pub clock_counter: u8,
    /// the current cpu state
    pub registers: NesRegisters,
}

impl Nes2a03 {
    pub fn get_cycles(&self) -> u64 {
        self.cycles
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
    fn get_status_stack(&self) -> u8 {
        let mut value = self.get_p();
        value |= FLAG_BL | FLAG_BH;
        value
    }
    fn set_status_stack(&mut self, value: u8) {
        let current_bh = self.get_p() & (FLAG_BH | FLAG_BL);
        let new_status = (value & !(FLAG_BH | FLAG_BL)) | current_bh;
        // println!("setting status {:08b}", new_status);
        self.set_p(new_status);
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
    fn set_decimal(&mut self) {
        self.set_p(self.get_p() | FLAG_DECIMAL);
    }
    fn clear_decimal(&mut self) {
        self.set_p(self.get_p() & !FLAG_DECIMAL);
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

impl RegisterAccess for Nes2a03 {
    fn get_a(&self) -> u8 {
        self.registers.a
    }
    fn get_x(&self) -> u8 {
        self.registers.x
    }
    fn get_y(&self) -> u8 {
        self.registers.y
    }
    fn get_p(&self) -> u8 {
        self.registers.p
    }
    fn get_sp(&self) -> u8 {
        self.registers.sp
    }
    fn get_pc(&self) -> u16 {
        self.registers.pc
    }

    fn set_a(&mut self, value: u8) -> u8 {
        let old = self.registers.a;
        self.registers.a = value;
        old
    }
    fn set_x(&mut self, value: u8) -> u8 {
        let old = self.registers.x;
        self.registers.x = value;
        old
    }
    fn set_y(&mut self, value: u8) -> u8 {
        let old = self.registers.y;
        self.registers.y = value;
        old
    }
    fn set_p(&mut self, value: u8) -> u8 {
        let old = self.registers.p;
        self.registers.p = value;
        old
    }
    fn set_sp(&mut self, value: u8) -> u8 {
        let old = self.registers.sp;
        self.registers.sp = value;
        old
    }
    fn set_pc(&mut self, value: u16) -> u16 {
        let old = self.registers.pc;
        self.registers.pc = value;
        old
    }
}
