use nesse_emu::peripherals::Spy;
use nesse_emu::prelude::*;
use std::time::Instant;

const ROM: &[u8] = include_bytes!("nestest.nes");
const LABEL_LIST: &[(u16, &str)] = &[];

// use awk to strip cycles info:
// cargo run --example test_cpu > out.txt
// cat out.txt | awk '{print substr($0,0, 73)}' > out_nocycle.log
// diff --left-column -W 154 -y out_nocycle.log nestest_nocycle.log > diff.txt

fn main() {
    let snake_cartridge = NesCart::from_slice(&ROM).expect("constant rom failed to load");

    let mut spy = Spy::new(LABEL_LIST);
    let mut two = FinalMemReader(0x02);
    let mut three = FinalMemReader(0x03);
    let mut timer = RunTime::new();
    let mut nes = Nes::default()
        // .with_peripheral(&mut printer)
        .with_peripheral(&mut spy)
        .with_peripheral(&mut two)
        .with_peripheral(&mut three)
        .with_peripheral(&mut timer);
    nes.insert_cartridge(snake_cartridge);
    nes.init();
    nes.set_pc(0xc000);

    nes.master_clock_drive();
    nes.cleanup();
}

pub struct FinalMemReader(u16);

impl NesPeripheral for FinalMemReader {
    fn cleanup(&mut self, nes: &mut Nes) {
        let value = nes.get(self.0);
        println!("0x{:04X} = 0x{:02X}", self.0, value);
    }
}

pub struct RunTime(Instant);

impl RunTime {
    fn new() -> Self {
        RunTime(Instant::now())
    }
}

impl NesPeripheral for RunTime {
    fn cleanup(&mut self, _nes: &mut Nes) {
        let current = Instant::now();
        let elapsed = current - self.0;
        println!("total run time: {:?}", elapsed);
    }
}
