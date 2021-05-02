use crate::Nes;

// generated in nesse_codegen
pub mod jumptable;

pub fn lda(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    let address = nes.get_address_from_mode(addressing);
    let value = nes.ram.get(address);
    nes.cpu.registers.a = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn tax(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    let value = nes.cpu.registers.a;
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn inx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    let value = nes.cpu.registers.x.wrapping_add(1);
    nes.cpu.registers.x = value;
    nes.cpu.registers.set_flags(value);
    cycles
}

pub fn brk(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    nes.cpu.registers.pc -= 1;
    nes.cpu.running = false;
    cycles
}

pub fn adc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    let carry = nes.cpu.registers.get_carry();
    let address = nes.get_address_from_mode(addressing);
    let input = nes.ram.get(address);
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
    cycles
}

// stub implementations provided by codegen crate:
pub fn cpy(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "cpy");
    nes.cpu.running = false;
    cycles
}

pub fn and(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "and");
    nes.cpu.running = false;
    cycles
}
pub fn asl(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "asl");
    nes.cpu.running = false;
    cycles
}
pub fn bcc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bcc");
    nes.cpu.running = false;
    cycles
}
pub fn bcs(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bcs");
    nes.cpu.running = false;
    cycles
}
pub fn beq(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "beq");
    nes.cpu.running = false;
    cycles
}
pub fn bit(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bit");
    nes.cpu.running = false;
    cycles
}
pub fn bmi(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bmi");
    nes.cpu.running = false;
    cycles
}
pub fn bne(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bne");
    nes.cpu.running = false;
    cycles
}
pub fn bpl(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bpl");
    nes.cpu.running = false;
    cycles
}

pub fn bvc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bvc");
    nes.cpu.running = false;
    cycles
}
pub fn bvs(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "bvs");
    nes.cpu.running = false;
    cycles
}
pub fn clc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "clc");
    nes.cpu.running = false;
    cycles
}
pub fn cld(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "cld");
    nes.cpu.running = false;
    cycles
}
pub fn cli(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "cli");
    nes.cpu.running = false;
    cycles
}
pub fn clv(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "clv");
    nes.cpu.running = false;
    cycles
}
pub fn cmp(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "cmp");
    nes.cpu.running = false;
    cycles
}
pub fn cpx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "cpx");
    nes.cpu.running = false;
    cycles
}
pub fn dec(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "dec");
    nes.cpu.running = false;
    cycles
}
pub fn dex(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "dex");
    nes.cpu.running = false;
    cycles
}
pub fn dey(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "dey");
    nes.cpu.running = false;
    cycles
}
pub fn eor(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "eor");
    nes.cpu.running = false;
    cycles
}
pub fn inc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "inc");
    nes.cpu.running = false;
    cycles
}

pub fn iny(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "iny");
    nes.cpu.running = false;
    cycles
}
pub fn jmp(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "jmp");
    nes.cpu.running = false;
    cycles
}
pub fn jsr(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "jsr");
    nes.cpu.running = false;
    cycles
}
pub fn ldx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "ldx");
    nes.cpu.running = false;
    cycles
}
pub fn ldy(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "ldy");
    nes.cpu.running = false;
    cycles
}
pub fn lsr(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "lsr");
    nes.cpu.running = false;
    cycles
}
pub fn nop(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "nop");
    nes.cpu.running = false;
    cycles
}
pub fn ora(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "ora");
    nes.cpu.running = false;
    cycles
}
pub fn pha(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "pha");
    nes.cpu.running = false;
    cycles
}
pub fn php(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "php");
    nes.cpu.running = false;
    cycles
}
pub fn pla(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "pla");
    nes.cpu.running = false;
    cycles
}
pub fn plp(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "plp");
    nes.cpu.running = false;
    cycles
}
pub fn rol(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "rol");
    nes.cpu.running = false;
    cycles
}
pub fn ror(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "ror");
    nes.cpu.running = false;
    cycles
}
pub fn rti(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "rti");
    nes.cpu.running = false;
    cycles
}
pub fn rts(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "rts");
    nes.cpu.running = false;
    cycles
}
pub fn sbc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "sbc");
    nes.cpu.running = false;
    cycles
}
pub fn sec(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "sec");
    nes.cpu.running = false;
    cycles
}
pub fn sed(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "sed");
    nes.cpu.running = false;
    cycles
}
pub fn sei(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "sei");
    nes.cpu.running = false;
    cycles
}
pub fn sta(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "sta");
    nes.cpu.running = false;
    cycles
}
pub fn stx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "stx");
    nes.cpu.running = false;
    cycles
}
pub fn sty(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "sty");
    nes.cpu.running = false;
    cycles
}

pub fn tay(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "tay");
    nes.cpu.running = false;
    cycles
}
pub fn tsx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "tsx");
    nes.cpu.running = false;
    cycles
}
pub fn txa(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "txa");
    nes.cpu.running = false;
    cycles
}
pub fn txs(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "txs");
    nes.cpu.running = false;
    cycles
}
pub fn tya(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("{} unimplemented", "tya");
    nes.cpu.running = false;
    cycles
}
