use super::*;

#[test]
fn test_0xa9_lda_immidiate_load_data() {
    let mut nes = Nes::default();
    nes.inject_operation("a9 05 00");
    nes.step();
    let regs = nes.dump_registers();
    let mut expected = NesRegisters::default();
    expected.set_a(5);
    println!("result: {:?}", regs);
    assert!(regs.a == expected.a);
    assert!(regs.p == expected.p);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut nes = Nes::default();
    nes.inject_registers(NesRegisters::default().with_flags_from(-2i8 as u8));
    nes.inject_operation("a9 00 00");
    nes.step();
    let regs = nes.dump_registers();
    let expected = NesRegisters::default().with_flags_from(-2i8 as u8).with_flags_from(0).with_pc(1);
    println!("result: {:?}\nexpect: {:?}", regs, expected);
    assert!(regs.status_zero() == expected.status_zero());
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut nes = Nes::default();
    nes.inject_registers(NesRegisters::default().with_a(10));
    nes.inject_operation("aa 00");
    nes.step();
    let regs = nes.dump_registers();
    let expected = NesRegisters::default().with_x(10).with_flags_from(10);
    println!("result: {:?}\n{:?}", regs, expected);
    assert!(regs.x == expected.x);
    assert!(regs.p & 0b10 == expected.p & 0b10);
}

#[test]
fn test_5_ops_working_together() {
    let mut nes = Nes::default();
    nes.inject_operation("a9 c0 aa e8 00");
    nes.run_until_nop();
    let regs = nes.dump_registers();
    let expected = NesRegisters::default()
        .with_x(0xc1)
        .with_a(192)
        .with_flags_from(0xe8)
        .with_pc(4);
    println!("result: {:?}\n{:?}", regs, expected);
    assert!(regs == expected);
}

#[test]
fn test_inx_overflow() {
    let mut nes = Nes::default();
    nes.inject_registers(NesRegisters::default().with_x(0xff));
    nes.inject_operation("e8 e8 00");
    nes.run_until_nop();
    let regs = nes.dump_registers();
    let expected = NesRegisters::default()
        .with_x(0x1)
        .with_flags_from(0x1)
        .with_pc(2);
    println!("result: {:?}\n{:?}", regs, expected);
    assert!(regs == expected);
}
