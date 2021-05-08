use nesse_emu::prelude::*;
use nesse_emu::peripherals::Spy;

const ROM: &[u8] = include_bytes!("nestest.nes");
const LABEL_LIST: &[(u16, &str)] = &[];

fn main() {
    let snake_cartridge = NesCart::from_slice(&ROM).expect("constant rom failed to load");

    let mut spy = Spy::new(LABEL_LIST);

    let mut nes = Nes::default()
        // .with_peripheral(&mut printer)
        .with_peripheral(&mut spy);
    nes.insert_cartridge(snake_cartridge);
    nes.init();
    nes.set_pc(0xc000);

    nes.run_until_nop();
    nes.cleanup();
}
