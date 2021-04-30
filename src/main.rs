use nesse::*;
fn main() {
    let mut nes = nesse::Nes::default();
    nes.inject_operation("a9 c0 aa e8 00");
    nes.step();
    let regs = nes.display_registers();

    println!("result: {}", regs);
}
