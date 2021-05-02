use crossterm::{Command, ExecutableCommand, QueueableCommand, Result, cursor, event::{self, Event, KeyCode, KeyEvent}, execute, queue, style::{self, Colorize}, terminal::{
        self, enable_raw_mode, disable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
    }};
use std::io::{Stdout, Write, stdout};

use nesse_emu::prelude::*;

const game_code: &[u8] = &[
    0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
    0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
    0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
    0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3,
    0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
    0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
    0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
    0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
    0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
    0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
    0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
    0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
    0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
    0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
    0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
    0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
    0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
    0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
    0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
    0xea, 0xca, 0xd0, 0xfb, 0x60,
];

fn main() {
    let mut nes = Nes::default()
        .with_peripheral(Box::new(RandomNumberGenerator(0xfe)))
        .with_peripheral(Box::new(KeyboardInput(0xff, 0x00)))
        .with_peripheral(Box::new(SimpleScreen(0x200)))
        .with_peripheral(Box::new(RateLimiter))
        .with_peripheral(Box::new(PCPrinter))
        .with_initial_memory(0, &game_code)
        ;
    nes.init();

    nes.run_until_nop();
    nes.cleanup();
}

pub struct RateLimiter;


impl NesPeripheral for RateLimiter {
    fn tick(&mut self, nes: &mut Nes) {
        ::std::thread::sleep(std::time::Duration::new(0, 100_000_000)); // 16_666_666
    }
}

/// inserts a random number into the game every tick at the given address
pub struct RandomNumberGenerator(u16);

impl NesPeripheral for RandomNumberGenerator {
    fn tick(&mut self, nes: &mut Nes) {
        let num:u8 = rand::thread_rng().gen_range(1,16);
        nes.inject_memory_value(self.0, num);
    }
}

pub struct KeyboardInput(u16, u8);

impl NesPeripheral for KeyboardInput {
    fn tick(&mut self, nes: &mut Nes) {
        nes.inject_memory_value(self.0, self.1);
    }
}

pub struct PCPrinter;

impl NesPeripheral for PCPrinter {
    fn cleanup(&mut self, nes: &mut Nes) {
        let next_opcode = nes.next_byte();
        println!("next opcode: {:x}", next_opcode);
    }
}

/// a 32x32 screen
pub struct SimpleScreen(u16);

impl NesPeripheral for SimpleScreen {
    fn init(&mut self, nes: &mut Nes) {
        
        let mut stdout = stdout();
        // stdout.execute(ScrollUp(3)).unwrap();
        execute!(stdout, EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        // stdout.execute(DisableLineWrap).unwrap();
        stdout.execute(cursor::Hide).unwrap();
        // stdout
        //     .execute(terminal::Clear(terminal::ClearType::All))
        //     .unwrap();
    }
    fn tick(&mut self, nes: &mut Nes) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();
        for x in 0..34 {
            for y in 0..34 {
                stdout.queue(cursor::MoveTo(x, y)).unwrap();
                if x == 0 || x == 33 || y == 0 || y == 33 {
                    stdout.queue(style::Print("#")).unwrap(); // █
                } else {
                    let x = x - 1;
                    let y = y - 1;
                    let color = nes.extract_memory(self.0 + x * 32 + y);
                    if color == 4 {
                        // should be "green"
                        stdout
                            .queue(style::PrintStyledContent("x".green()))
                            .unwrap();
                    } else if color == 3 {
                        // should be "red"
                        stdout.queue(style::PrintStyledContent("o".red())).unwrap();
                    } else {
                        stdout.queue(style::Print(" ")).unwrap();
                    }
                }
            }
        }
        stdout.queue(cursor::MoveTo(0, 37)).unwrap();
        // stdout.queue(ScrollDown(2)).unwrap();
        stdout.flush().unwrap();
    }
    fn cleanup(&mut self, nes: &mut Nes) {
        let mut stdout = stdout();
        // stdout.execute(EnableLineWrap).unwrap();
        
        disable_raw_mode().unwrap();
        execute!(stdout, LeaveAlternateScreen).unwrap();
        stdout.execute(cursor::Show).unwrap();
        
        
    }
}

