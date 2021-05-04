use crate::{AddressableMemory, Nes};

// generated in nesse_codegen
pub mod jumptable;
pub mod opcode_debug;

// load & store family --------------------------------------------------------
pub fn ldx(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn lda(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    nes.cpu.registers.a = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn ldy(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    nes.cpu.registers.y = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn sta(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    // println!("storing value of a {} in location {}", nes.cpu.registers.a, address);
    nes.set(address, nes.cpu.registers.a);
    cycles
}

// registers-only family -------------------------------------------------
pub fn tax(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.a;
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    cycles
}
pub fn txa(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.x;
    nes.cpu.registers.a = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn inx(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.x.wrapping_add(1);
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    // println!("inx to {}", value);
    cycles
}
pub fn dex(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.x.wrapping_sub(1);
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    // println!("dex to {}", value);
    cycles
}

pub fn clc(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    nes.cpu.registers.clear_carry();
    cycles
}

pub fn sec(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    nes.cpu.registers.set_carry();
    cycles
}

// interrupts family ------------------------------------------------------
pub fn brk(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    nes.cpu.registers.pc -= 1;
    nes.cpu.running = false;
    cycles
}

pub fn nop(_nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    cycles
}

// arithmetic family -----------------------------------------------------

/// helper function to add a value to register a, setting appropriate flags
/// including the carry flag
fn add_to_a(nes: &mut Nes, input: u8) {
    let carry = nes.cpu.registers.get_carry();
    let value = input as u16 + nes.cpu.registers.a as u16 + carry as u16;
    let low_value = (value & 0xff) as u8;

    nes.cpu.registers.set_flags(low_value);
    if value > 0xff {
        nes.cpu.registers.set_carry();
    } else {
        nes.cpu.registers.clear_carry();
    }
    if (input ^ low_value) & (low_value ^ nes.cpu.registers.a) & 0x80 != 0 {
        nes.cpu.registers.set_overflow();
    } else {
        nes.cpu.registers.clear_overflow();
    }
    nes.cpu.registers.a = low_value;
}

pub fn adc(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let input = nes.get(address);
    add_to_a(nes, input);
    cycles
}

pub fn sbc(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let input = nes.get(address) as i8;
    add_to_a(nes, input.wrapping_neg().wrapping_sub(1) as u8);
    cycles
}

pub fn and(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address) & nes.cpu.registers.a;
    nes.cpu.registers.a = value;
    nes.cpu.registers.set_flags(value);
    cycles
}
pub fn lsr(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    // todo: this is gross, fix it fix it fix it
    if addressing == 1 {
        // operate on the accumulator
        let mut value = nes.cpu.registers.a;
        if value & 0b1 == 0b1 {
            nes.cpu.registers.set_carry();
        }
        value = value >> 1;
        nes.cpu.registers.set_flags(value);
        nes.cpu.registers.a = value;
    } else {
        // operate per addressing mode
        let address = nes.get_address_from_mode(addressing);
        let mut value = nes.get(address);
        if value & 0b1 == 0b1 {
            nes.cpu.registers.set_carry();
        }
        value = value >> 1;
        nes.cpu.registers.set_flags(value);
        // println!("left shifted value {} in memory address {}", value, address);
        nes.set(address, value);
    }
    cycles
}

pub fn dec(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address).wrapping_sub(1);
    // println!("decremented value {} in address {}", value, address);
    nes.set(address, value);
    nes.cpu.registers.set_flags(value);
    cycles
}

// compare family --------------------------------------------------------------
pub fn cmp(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    let compare = nes.cpu.registers.a as i16 - value as i16;
    if compare <= 0 {
        nes.cpu.registers.set_carry();
    }
    nes.cpu.registers.set_flags(compare as u8);
    // println!("cmp {} - {} resulted in {}", nes.cpu.registers.a, value, compare);
    cycles
}
pub fn cpx(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    // println!("comparing {} to {}", value, nes.cpu.registers.x);
    let compare = nes.cpu.registers.x as i16 - value as i16;
    if compare <= 0 {
        nes.cpu.registers.set_carry();
    }
    nes.cpu.registers.set_flags(compare as u8);
    cycles
}

pub fn bit(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    let mask = nes.cpu.registers.a;
    let result = value & mask;

    nes.cpu.registers.set_overflow_from(value);
    nes.cpu.registers.set_negative_from(value);
    nes.cpu.registers.set_zero_from(result);

    cycles
}

// jump & branch family ----------------------------------------------------------------
pub fn jsr(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let jump_address = nes.get_short(nes.cpu.registers.pc);
    // todo: validate this stays correct with tests
    // skip two bytes because above call didn't advance PC when reading 2 bytes
    // but subtract one from the value because its a quirk of the cpu evidently
    let return_address = nes.cpu.registers.pc + 2 - 1;

    nes.stack_push_short(return_address);
    nes.cpu.registers.pc = jump_address;
    println!("jumping to subroutine {:x}", jump_address);
    cycles
}
pub fn jmp(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let jump_address = nes.get_short(address);
    nes.cpu.registers.pc = jump_address;
    println!("jumping to {:x}", jump_address);
    cycles
}
pub fn rts(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.stack_pop_short();
    // add one back to the value we got since we subtracted one in jsr
    // todo: add tests to show this has the right value
    let pc = value.wrapping_add(1);
    nes.cpu.registers.pc = pc;
    println!("returning to {:x}", pc);
    cycles
}

pub fn beq(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    // // todo: this is always relative addressing
    // // let address = nes.get_address_from_mode(addressing);
    // let base = nes.get(nes.cpu.registers.pc) as u16;
    // nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    // if nes.cpu.registers.status_zero() == true {
    //     let pc = nes.cpu.registers.pc
    //         .wrapping_add(base);
    //         // .wrapping_add(1);
    //     nes.cpu.registers.pc = pc;
    //     println!("beqranching to {:x}", pc);
    //     // todo: cycles should increment
    // }
    let offset = nes.get(nes.cpu.registers.pc) as u16;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_zero() == true {
        let pc = nes.cpu.registers.pc.wrapping_add(offset);
        nes.cpu.registers.pc = pc;
        println!("beqranching to {:x}", pc);
    }
    cycles
}

pub fn bne(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    if nes.cpu.registers.status_zero() == false {
        let offset = nes.get(nes.cpu.registers.pc) as i8;
        // println!("adding {} to pc {}", offset, nes.cpu.registers.pc);
        let new_pc = nes.cpu.registers.pc
        .wrapping_add(1) // add one to advance PC past the above read
        as i32; // upcast to i32 in order to avoid clipping
        let pc = ((new_pc + offset as i32) & 0xffff) as u16;
        nes.cpu.registers.pc = pc;
        println!("bneranching to {:x}", pc);
    } else {
        nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    }
    cycles
}

pub fn bpl(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let offset = nes.get(nes.cpu.registers.pc) as i8;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_negative() == false {
        // println!("adding {} to pc {}", offset, nes.cpu.registers.pc);
        let mut pc = nes.cpu.registers.pc as i32;
        pc = (pc + offset as i32) & 0xffff;
        nes.cpu.registers.pc = pc as u16;
        println!("bplranching to {:x}", pc);
    }
    cycles
}

pub fn bcs(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let offset = nes.get(nes.cpu.registers.pc) as i8;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_carry() == true {
        // println!("adding {} to pc {}", offset, nes.cpu.registers.pc);
        let mut pc = nes.cpu.registers.pc as i32;
        pc = (pc + offset as i32) & 0xffff;
        nes.cpu.registers.pc = pc as u16;
        println!("bcsranching to {:x}", pc);
    }
    cycles
}

pub fn bcc(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let offset = nes.get(nes.cpu.registers.pc) as i8;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_carry() == false {
        // println!("adding {} to pc {}", offset, nes.cpu.registers.pc);
        let mut pc = nes.cpu.registers.pc as i32;
        pc = (pc + offset as i32) & 0xffff;
        nes.cpu.registers.pc = pc as u16;
        println!("bccranching to {:x}", pc);
    }
    cycles
}

// stub implementations provided by codegen crate:
pub fn cpy(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "cpy");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn asl(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented: {}", "asl", addressing);
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn bmi(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bmi");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn bvc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bvc");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn bvs(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bvs");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn cld(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "cld");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn cli(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "cli");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn clv(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "clv");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn dey(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "dey");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn eor(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "eor");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn inc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "inc");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn iny(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "iny");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn ora(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "ora");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn pha(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "pha");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn php(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "php");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn pla(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "pla");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn plp(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "plp");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn rol(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "rol");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn ror(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "ror");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn rti(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "rti");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn sed(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "sed");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn sei(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "sei");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn stx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "stx");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn sty(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "sty");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn tay(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "tay");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn tsx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "tsx");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}

pub fn txs(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "txs");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
pub fn tya(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "tya");
    nes.cpu.running = false;
    nes.cpu.registers.pc -= 1;
    cycles
}
