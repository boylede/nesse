use nesse_emu::*;

fn main() {
    let mut nes = nesse_emu::Nes::default();
    nes.inject_operation("a9 c0 aa e8 00");
    nes.run_until_nop();
    let regs = nes.display_registers();

    println!("result: {}", regs);
}
