// D659 is first failure

use crate::{AddressableMemory, Nes};

// generated in nesse_codegen:
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
    nes.set(address, nes.cpu.registers.a);

    cycles
}

pub fn stx(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    nes.set(address, nes.cpu.registers.x);

    cycles
}

pub fn sty(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    nes.set(address, nes.cpu.registers.y);

    cycles
}

// stack-related loads and stores ---------------------------------------

/// push p to stack
pub fn php(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.get_status_stack();
    nes.stack_push(value);
    cycles
}

/// pull p from stack
pub fn plp(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.stack_pop();
    nes.cpu.registers.set_status_stack(value);
    cycles
}

/// pull a from stack
pub fn pla(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.stack_pop();
    nes.cpu.registers.set_flags(value);
    nes.cpu.registers.set_a(value);
    cycles
}

/// push a to stack
pub fn pha(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.a;
    nes.stack_push(value);
    cycles
}

/// transfer stackpointer to x
pub fn tsx(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.sp;
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

/// transfer x to stackpointer
pub fn txs(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.x;
    nes.cpu.registers.sp = value;
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

pub fn tay(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.a;
    nes.cpu.registers.y = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn tya(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.y;
    nes.cpu.registers.a = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn inx(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.x.wrapping_add(1);
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    cycles
}
pub fn iny(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.y.wrapping_add(1);
    nes.cpu.registers.y = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn dex(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.x.wrapping_sub(1);
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    cycles
}
pub fn dey(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = nes.cpu.registers.y.wrapping_sub(1);
    nes.cpu.registers.y = value;
    nes.cpu.registers.set_flags(value);
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

pub fn sed(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    nes.cpu.registers.set_decimal();
    cycles
}

pub fn cld(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    nes.cpu.registers.clear_decimal();
    cycles
}

pub fn clv(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    nes.cpu.registers.clear_overflow();
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

pub fn sei(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    nes.cpu.registers.set_interrupt();
    cycles
}
pub fn cli(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    nes.cpu.registers.clear_interrupt();
    cycles
}

/// return from interrupt
pub fn rti(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let flags = nes.stack_pop();
    let value = nes.stack_pop_short();
    // add one back to the value we got since we subtracted one in jsr
    // let pc = value.wrapping_add(1); // todo: why dont we add 1 here like rts?
    nes.cpu.registers.pc = value;
    nes.cpu.registers.set_status_stack(flags);

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
        } else {
            nes.cpu.registers.clear_carry();
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
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value >> 1;
        nes.cpu.registers.set_flags(value);
        nes.set(address, value);
    }
    cycles
}

pub fn dec(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address).wrapping_sub(1);
    nes.set(address, value);
    nes.cpu.registers.set_flags(value);
    cycles
}
pub fn inc(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address).wrapping_add(1);
    nes.set(address, value);
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn ora(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address) | nes.cpu.registers.a;
    nes.cpu.registers.a = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn eor(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address) ^ nes.cpu.registers.a;
    nes.cpu.registers.a = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn asl(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    if addressing == 1 {
        // operate on the accumulator
        let mut value = nes.cpu.registers.a;
        if value & 1 << 7 == 1 << 7 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value << 1;
        nes.cpu.registers.set_flags(value);
        nes.cpu.registers.a = value;
    } else {
        // operate per addressing mode
        let address = nes.get_address_from_mode(addressing);
        let mut value = nes.get(address);
        if value & 1 << 7 == 1 << 7 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value << 1;
        nes.cpu.registers.set_flags(value);
        nes.set(address, value);
    }
    cycles
}

pub fn ror(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    if addressing == 1 {
        // operate on the accumulator
        let mut value = nes.cpu.registers.a;
        let carry = nes.cpu.registers.get_carry() << 7;
        if value & 1 == 1 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value >> 1;
        value |= carry;
        nes.cpu.registers.set_flags(value);
        nes.cpu.registers.a = value;
    } else {
        // operate per addressing mode
        let address = nes.get_address_from_mode(addressing);
        let mut value = nes.get(address);
        let carry = nes.cpu.registers.get_carry() << 7;
        if value & 1 == 1 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value >> 1;
        value |= carry;
        nes.cpu.registers.set_flags(value);
        nes.set(address, value);
    }
    cycles
}

pub fn rol(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    if addressing == 1 {
        // operate on the accumulator
        let mut value = nes.cpu.registers.a;
        let carry = nes.cpu.registers.get_carry();
        if value & 1 << 7 == 1 << 7 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value << 1;
        value |= carry;
        nes.cpu.registers.set_flags(value);
        nes.cpu.registers.a = value;
    } else {
        // operate per addressing mode
        let address = nes.get_address_from_mode(addressing);
        let mut value = nes.get(address);
        let carry = nes.cpu.registers.get_carry();
        if value & 1 << 7 == 1 << 7 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value << 1;
        value |= carry;
        nes.cpu.registers.set_flags(value);
        nes.set(address, value);
    }
    cycles
}

// compare family --------------------------------------------------------------
pub fn cmp(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    let compare = nes.cpu.registers.a as i16 - value as i16;
    if compare >= 0 {
        nes.cpu.registers.set_carry();
    } else {
        nes.cpu.registers.clear_carry();
    }
    nes.cpu.registers.set_flags(compare as u8);
    cycles
}
pub fn cpx(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    let compare = nes.cpu.registers.x as i16 - value as i16;
    if compare >= 0 {
        nes.cpu.registers.set_carry();
    } else {
        nes.cpu.registers.clear_carry();
    }
    nes.cpu.registers.set_flags(compare as u8);
    cycles
}
pub fn cpy(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    let compare = nes.cpu.registers.y as i16 - value as i16;
    if compare >= 0 {
        nes.cpu.registers.set_carry();
    } else {
        nes.cpu.registers.clear_carry();
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
    cycles
}
pub fn jmp(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    nes.cpu.registers.pc = address;
    cycles
}
pub fn rts(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    if nes.cpu.registers.sp != super::STACK_INITIAL {
        let value = nes.stack_pop_short();
        // add one back to the value we got since we subtracted one in jsr
        // todo: add tests to show this has the right value
        let pc = value.wrapping_add(1);
        nes.cpu.registers.pc = pc;
        cycles
    } else {
        // main loop is over?
        println!("rts resulted in stack underflow. stopping.");
        nes.cpu.registers.pc -= 1;
        nes.cpu.running = false;
        cycles
    }
    
}

pub fn beq(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let offset = nes.get(nes.cpu.registers.pc) as u16;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_zero() == true {
        let pc = nes.cpu.registers.pc.wrapping_add(offset);
        nes.cpu.registers.pc = pc;
    }
    cycles
}

pub fn bne(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    if nes.cpu.registers.status_zero() == false {
        let offset = nes.get(nes.cpu.registers.pc) as i8;
        let new_pc = nes.cpu.registers.pc
        .wrapping_add(1) // add one to advance PC past the above read
        as i32; // upcast to i32 in order to avoid clipping
        let pc = ((new_pc + offset as i32) & 0xffff) as u16;
        nes.cpu.registers.pc = pc;
    } else {
        nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    }
    cycles
}

pub fn bpl(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let offset = nes.get(nes.cpu.registers.pc) as i8;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_negative() == false {
        let mut pc = nes.cpu.registers.pc as i32;
        pc = (pc + offset as i32) & 0xffff;
        nes.cpu.registers.pc = pc as u16;
    }
    cycles
}

pub fn bcs(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    if nes.cpu.registers.status_carry() == true {
        let offset = nes.get(nes.cpu.registers.pc) as i8;
        let mut pc = nes.cpu.registers.pc.wrapping_add(1) as i32; // increment......
        pc = (pc + (offset as i32)) & 0xffff;
        nes.cpu.registers.pc = pc as u16;
    } else {
        nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    }
    cycles
}

pub fn bcc(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let offset = nes.get(nes.cpu.registers.pc) as i8;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_carry() == false {
        let mut pc = nes.cpu.registers.pc as i32;
        pc = (pc + offset as i32) & 0xffff;
        nes.cpu.registers.pc = pc as u16;
    }
    cycles
}

pub fn bvs(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let offset = nes.get(nes.cpu.registers.pc) as i8;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_overflow() == true {
        let mut pc = nes.cpu.registers.pc as i32;
        pc = (pc + offset as i32) & 0xffff;
        nes.cpu.registers.pc = pc as u16;
    }
    cycles
}

pub fn bvc(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let offset = nes.get(nes.cpu.registers.pc) as i8;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_overflow() == false {
        let mut pc = nes.cpu.registers.pc as i32;
        pc = (pc + offset as i32) & 0xffff;
        nes.cpu.registers.pc = pc as u16;
    }
    cycles
}

pub fn bmi(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let offset = nes.get(nes.cpu.registers.pc) as i8;
    nes.cpu.registers.pc = nes.cpu.registers.pc.wrapping_add(1);
    if nes.cpu.registers.status_negative() == true {
        let mut pc = nes.cpu.registers.pc as i32;
        pc = (pc + offset as i32) & 0xffff;
        nes.cpu.registers.pc = pc as u16;
    }
    cycles
}
/////////////////////////////////////////////////////////////////////////////
// extra / extended / undocumented or unofficial opcodes
pub fn ign(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let _ = nes.get_address_from_mode(addressing);
    cycles
}
pub fn lax(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address);
    nes.cpu.registers.a = value;
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn sax(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.cpu.registers.x & nes.cpu.registers.a;
    nes.set(address, value);
    cycles
}

pub fn dcp(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    //// dec
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address).wrapping_sub(1);
    nes.set(address, value);
    // nes.cpu.registers.set_flags(value);
    //// cmp
    // let address = nes.get_address_from_mode(addressing);
    // let value = nes.get(address);
    let compare = nes.cpu.registers.a as i16 - value as i16;
    if compare >= 0 {
        nes.cpu.registers.set_carry();
    } else {
        nes.cpu.registers.clear_carry();
    }
    nes.cpu.registers.set_flags(compare as u8);
    cycles
}
/// aka ISB
pub fn isc(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    // inc
    let address = nes.get_address_from_mode(addressing);
    let value = nes.get(address).wrapping_add(1);
    nes.set(address, value);
    nes.cpu.registers.set_flags(value);
    // sbc
    let input = value as i8;
    add_to_a(nes, input.wrapping_neg().wrapping_sub(1) as u8);
    cycles
}

// ASL value then ORA
pub fn slo(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let value = if addressing == 1 {
        // operate on the accumulator
        let mut value = nes.cpu.registers.a;
        if value & 1 << 7 == 1 << 7 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value << 1;
        nes.cpu.registers.a = value;
        value
    } else {
        // operate per addressing mode
        let address = nes.get_address_from_mode(addressing);
        let mut value = nes.get(address);
        if value & 1 << 7 == 1 << 7 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value << 1;
        nes.set(address, value);
        value
    };
    let result = value | nes.cpu.registers.a;
    nes.cpu.registers.a = result;
    nes.cpu.registers.set_flags(result);
    cycles
}

/// ROL value then AND value
pub fn rla(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    // ROL
    let value = if addressing == 1 {
        // operate on the accumulator
        let mut value = nes.cpu.registers.a;
        let carry = nes.cpu.registers.get_carry();
        if value & 1 << 7 == 1 << 7 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value << 1;
        value |= carry;
        nes.cpu.registers.a = value;
        value
    } else {
        // operate per addressing mode
        let address = nes.get_address_from_mode(addressing);
        let mut value = nes.get(address);
        let carry = nes.cpu.registers.get_carry();
        if value & 1 << 7 == 1 << 7 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value << 1;
        value |= carry;
        nes.set(address, value);
        value
    };
    // AND
    let result = nes.cpu.registers.a & value;
    nes.cpu.registers.a = result;
    nes.cpu.registers.set_flags(result);
    cycles
}
// LSR value then EOR
pub fn sre(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    // lsr
    let value = if addressing == 1 {
        // operate on the accumulator
        let mut value = nes.cpu.registers.a;
        if value & 0b1 == 0b1 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value >> 1;
        nes.cpu.registers.a = value;
        value
    } else {
        // operate per addressing mode
        let address = nes.get_address_from_mode(addressing);
        let mut value = nes.get(address);
        if value & 0b1 == 0b1 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value >> 1;
        nes.set(address, value);
        value
    };
    // eor
    let result = value ^ nes.cpu.registers.a;
    nes.cpu.registers.a = result;
    nes.cpu.registers.set_flags(result);
    cycles
}

// ROR value then ADC
pub fn rra(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    // ror
    let value = if addressing == 1 {
        // operate on the accumulator
        let mut value = nes.cpu.registers.a;
        let carry = nes.cpu.registers.get_carry() << 7;
        if value & 1 == 1 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value >> 1;
        value |= carry;
        
        nes.cpu.registers.a = value;
        value
    } else {
        // operate per addressing mode
        let address = nes.get_address_from_mode(addressing);
        let mut value = nes.get(address);
        let carry = nes.cpu.registers.get_carry() << 7;
        if value & 1 == 1 {
            nes.cpu.registers.set_carry();
        } else {
            nes.cpu.registers.clear_carry();
        }
        value = value >> 1;
        value |= carry;
        
        nes.set(address, value);
        value
    };
    add_to_a(nes, value);
    cycles
}

pub fn halt(nes: &mut Nes, _addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    nes.cpu.registers.pc -= 1;
    nes.cpu.running = false;
    cycles
}

//////////////////////////////////////////////////////////////////////////////
// unimplemented opcodes
pub fn alr(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let _ = nes.get_address_from_mode(addressing);
    cycles
}
pub fn anc(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let _ = nes.get_address_from_mode(addressing);
    cycles
}
pub fn arr(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let _ = nes.get_address_from_mode(addressing);
    cycles
}
pub fn axs(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let _ = nes.get_address_from_mode(addressing);
    cycles
}

pub fn skb(nes: &mut Nes, addressing: u8, cycles: u8, _bytes: u8) -> u8 {
    let _ = nes.get_address_from_mode(addressing);
    cycles
}
