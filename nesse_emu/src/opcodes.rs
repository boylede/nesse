use crate::Nes;
use nesse_common::{
    AddressingMode, CompactOpcode, CyclesCost, NesMetaOpcode, NesOpcode, StatusFlags, StatusOption,
};

// generated in nesse_codegen
pub mod jumptable;

pub fn lda(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    // let addr = self.get_operand_address(&mode);
    // let value = self.mem_read(addr);
    // self.set_register_a(value);
    println!("stub of lda");
}

// stub implementations provided by codegen crate:
pub fn adc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "adc");
    unimplemented!()
}
pub fn and(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "and");
    unimplemented!()
}
pub fn asl(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "asl");
    unimplemented!()
}
pub fn bcc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "bcc");
    unimplemented!()
}
pub fn bcs(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "bcs");
    unimplemented!()
}
pub fn beq(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "beq");
    unimplemented!()
}
pub fn bit(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "bit");
    unimplemented!()
}
pub fn bmi(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "bmi");
    unimplemented!()
}
pub fn bne(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "bne");
    unimplemented!()
}
pub fn bpl(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "bpl");
    unimplemented!()
}
pub fn brk(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "brk");
    unimplemented!()
}
pub fn bvc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "bvc");
    unimplemented!()
}
pub fn bvs(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "bvs");
    unimplemented!()
}
pub fn clc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "clc");
    unimplemented!()
}
pub fn cld(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "cld");
    unimplemented!()
}
pub fn cli(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "cli");
    unimplemented!()
}
pub fn clv(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "clv");
    unimplemented!()
}
pub fn cmp(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "cmp");
    unimplemented!()
}
pub fn cpx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "cpx");
    unimplemented!()
}
pub fn cpy(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "cpy");
    unimplemented!()
}
pub fn dec(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "dec");
    unimplemented!()
}
pub fn dex(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "dex");
    unimplemented!()
}
pub fn dey(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "dey");
    unimplemented!()
}
pub fn eor(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "eor");
    unimplemented!()
}
pub fn inc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "inc");
    unimplemented!()
}
pub fn inx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "inx");
    unimplemented!()
}
pub fn iny(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "iny");
    unimplemented!()
}
pub fn jmp(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "jmp");
    unimplemented!()
}
pub fn jsr(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "jsr");
    unimplemented!()
}
pub fn ldx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "ldx");
    unimplemented!()
}
pub fn ldy(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "ldy");
    unimplemented!()
}
pub fn lsr(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "lsr");
    unimplemented!()
}
pub fn nop(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "nop");
    unimplemented!()
}
pub fn ora(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "ora");
    unimplemented!()
}
pub fn pha(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "pha");
    unimplemented!()
}
pub fn php(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "php");
    unimplemented!()
}
pub fn pla(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "pla");
    unimplemented!()
}
pub fn plp(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "plp");
    unimplemented!()
}
pub fn rol(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "rol");
    unimplemented!()
}
pub fn ror(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "ror");
    unimplemented!()
}
pub fn rti(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "rti");
    unimplemented!()
}
pub fn rts(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "rts");
    unimplemented!()
}
pub fn sbc(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "sbc");
    unimplemented!()
}
pub fn sec(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "sec");
    unimplemented!()
}
pub fn sed(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "sed");
    unimplemented!()
}
pub fn sei(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "sei");
    unimplemented!()
}
pub fn sta(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "sta");
    unimplemented!()
}
pub fn stx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "stx");
    unimplemented!()
}
pub fn sty(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "sty");
    unimplemented!()
}
pub fn tax(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "tax");
    unimplemented!()
}
pub fn tay(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "tay");
    unimplemented!()
}
pub fn tsx(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "tsx");
    unimplemented!()
}
pub fn txa(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "txa");
    unimplemented!()
}
pub fn txs(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "txs");
    unimplemented!()
}
pub fn tya(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) {
    println!("{} unimplemented", "tya");
    unimplemented!()
}
