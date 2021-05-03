use nesse_emu::prelude::*;
use rand::Rng;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    EventPump, Sdl, VideoSubsystem,
};

#[rustfmt::skip]
const GAME_CODE: &[u8] = &[
    // 0x0600
    0x20, 0x06, 0x06, // jsr init
    0x20, 0x38, 0x06, // jsr loop
    // 0x0606 init:
    0x20, 0x0d, 0x06, // jsr init_snake
    0x20, 0x2a, 0x06, // jsr generate_apple_position
    0x60, // rts
    // 0x060d init_snake
    0xa9, 0x02, // lda 2 (snake direction)
    0x85, 0x02, // sta 0x0002
    0xa9, 0x04, // lda 4 (length of snake)
    0x85, 0x03, // sta 0x0003
    0xa9, 0x11, // lda 17 (something about head position)
    0x85, 0x10, // sta 0x0010
    0xa9, 0x10, // lda 16 (something about snake body)
    0x85, 0x12, // sta 0x0012
    0xa9, 0x0f, // lda 15
    0x85, 0x14, // sta 0x0014
    0xa9, 0x04, // lda 4
    0x85, 0x11, // sta 0x0011
    0x85, 0x13, // sta 0x0013
    0x85, 0x15, // sta 0x0015
    0x60, // rts
    // 0x0602a generate_apple_position
    0xa5, 0xfe, // lda
    0x85, 0x00, // sta
    0xa5, 0xfe, // lda
    0x29, 0x03, // and
    0x18, // clc
    0x69, 0x02,
    0x85, 0x01,
    0x60, // rts
    // 0x0638 loop
    0x20, 0x4d, 0x06, // jsr readKeys
    0x20, 0x8d, 0x06, // jsr checkCollision
    0x20, 0xc3, 0x06, // jsr updateSnake
    0x20, 0x19, 0x07, // jsr drawApple
    0x20, 0x20, 0x07, // jsr drawSnake
    0x20, 0x2d, 0x07, // jsr spinWheels
    0x4c, 0x38, 0x06, // jmp loop
    // 0x064d readkeys
    0xa5, 0xff, //lda
    0xc9, 0x77, // cmp
    0xf0, 0x0d, // beq up_key
    0xc9, 0x64,
    0xf0, 0x14, // beq right_key
    0xc9, 0x73,
    0xf0, 0x1b, // beq down_key
    0xc9, 0x61,
    0xf0, 0x22, // beq left_key
    0x60, // rts
    // up_key
    0xa9, 0x04,
    0x24, 0x02, 
    0xd0, 0x26, // bne illegal_move
    0xa9, 0x01,
    0x85, 0x02,
    0x60, // rts
    // right_key
    0xa9, 0x08,
    0x24, 0x02, 
    0xd0, 0x1b, // bne illegal_move
    0xa9, 0x02,
    0x85, 0x02,
    0x60, // rts
    // down_key
    0xa9, 0x01,
    0x24, 0x02,
    0xd0, 0x10, // bne illegal_move
    0xa9, 0x04,
    0x85, 0x02,
    0x60, // rts
    // left_key
    0xa9, 0x02,
    0x24, 0x02,
    0xd0, 0x05, // bne illegal_move
    0xa9, 0x08,
    0x85, 0x02,
    0x60, // rts
    // illegal_move
    0x60, // rts
    // check_collision
    0x20, 0x94, 0x06, // jsr check_apple_collission
    0x20, 0xa8, 0x06, // jsr check_snake_collision
    0x60, // rts
    // 0x0694 check_apple_collission
    0xa5, 0x00, // lda
    0xc5, 0x10, 0xd0, 0x0d,
    0xa5, 0x01, // lda
    0xc5, 0x11, 0xd0, 0x07,
    0xe6, 0x03, 0xe6, 0x03,
    0x20, 0x2a, 0x06, // jsr
    0x60, // rts
    0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
    0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
    0x35, 0x07,
    0x60, // rts
    0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9,
    0xa5, 0x02, // lda
    0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f,
    0xa5, 0x10, // lda
    0x38, 0xe9,
    0x20, 0x85, 0x10, // jsr
    0x90, 0x01,
    0x60, // rts
    0xc6, 0x11,
    0xa9, 0x01,
    0xc5, 0x11, 0xf0, 0x28,
    0x60, // rts
    0xe6,
    0x10,
    0xa9, 0x1f,
    0x24, 0x10, 0xf0, 0x1f,
    0x60,
    0xa5, 0x10, // lda
    0x18, 0x69,
    0x20, 0x85, 0x10, // jsr
    0xb0,
    0x01,
    0x60, // rts
    0xe6, 0x11,
    0xa9, 0x06,
    0xc5, 0x11, 0xf0, 0x0c,
    0x60, // rts
    0xc6, 0x10,
    0xa5, 0x10, // lda
    0x29,
    0x1f, 0xc9, 0x1f, 0xf0, 0x01,
    0x60,
    0x4c, 0x35, 0x07, 0xa0, 0x00,
    0xa5, 0xfe, // lda
    0x91, 0x00, 
    0x60, // rts
    0xa6, 0x03,
    0xa9, 0x00,
    0x81, 0x10, 0xa2, 0x00,
    0xa9, 0x01,
    0x81, 0x10, 
    0x60, // rts
    // spinwheels
    0xa2, 0x00,  // ldx 0
    // spinloop
    0xea, // nop
    0xea, // nop
    0xca, // dex
    0xd0, 0xfb, // bne spinloop
    0x60, // rts
];

fn main() {
    // let snake_cartridge = NesCart::simple(0x600, game_code);
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem
        .window("Snake game", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
        .unwrap();

    let mut random = RandomNumberGenerator(0xfe);
    let mut input = KeyboardInput::new(0xff, event_pump);
    let mut screen = SimpleScreen::new(0x200, texture, &mut canvas);
    let mut spy = Spy;
    let mut rate = RateLimiter;

    let mut nes = Nes::default()
        .with_peripheral(&mut random)
        .with_peripheral(&mut input)
        .with_peripheral(&mut spy)
        .with_peripheral(&mut screen)
        .with_peripheral(&mut rate)
        // .with_peripheral(Box::new(PCPrinter))
        .with_initial_memory(0x600, &GAME_CODE);
    nes.set_pc(0x600);
    nes.init();

    nes.run_until_nop();
    nes.cleanup();
}

pub struct RateLimiter;

impl NesPeripheral for RateLimiter {
    fn tick(&mut self, nes: &mut Nes) {
        ::std::thread::sleep(std::time::Duration::new(0, 10_000_000)); // 16_666_666
    }
}

pub struct Spy;

impl NesPeripheral for Spy {
    fn tick(&mut self, nes: &mut Nes) {
        let next_opcode = nes.peek_pc();
        let stack = nes.dump_stack();
        print!("{:x} ## {:?} ", next_opcode, nes.dump_registers());
        println!("{}", stack);
    }
}

/// inserts a random number into the game every tick at the given address
pub struct RandomNumberGenerator(u16);

impl NesPeripheral for RandomNumberGenerator {
    fn tick(&mut self, nes: &mut Nes) {
        let num: u8 = rand::thread_rng().gen_range(1, 16);
        nes.inject_memory_value(self.0, num);
    }
}

pub struct KeyboardInput(u16, EventPump);

impl KeyboardInput {
    pub fn new(mapped_address: u16, event_pump: EventPump) -> KeyboardInput {
        KeyboardInput(mapped_address, event_pump)
    }
}

impl NesPeripheral for KeyboardInput {
    fn tick(&mut self, nes: &mut Nes) {
        for event in self.1.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    std::process::exit(0)
                },
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    nes.inject_memory_value(self.0, 0x77);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    nes.inject_memory_value(self.0, 0x73);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    nes.inject_memory_value(self.0, 0x61);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    nes.inject_memory_value(self.0, 0x64);
                }
                _ => {
                    // ignore other keys
                }
            }
        }
    }
}

pub struct PCPrinter;

impl NesPeripheral for PCPrinter {
    fn cleanup(&mut self, nes: &mut Nes) {
        let next_opcode = nes.peek_pc();
        println!(
            "next opcode: {:x} at location {:?}",
            next_opcode,
            nes.dump_registers()
        );
    }
}

/// a 32x32 screen
pub struct SimpleScreen<'a> {
    mapped_address: u16,
    last_screen_state: Vec<u8>,
    texture: Texture<'a>,
    canvas: &'a mut Canvas<Window>,
}

impl<'a> SimpleScreen<'a> {
    pub fn new(
        mapped_address: u16,
        texture: Texture<'a>,
        canvas: &'a mut Canvas<Window>,
    ) -> SimpleScreen<'a> {
        let last_screen_state = vec![0u8; 32 * 32];
        SimpleScreen {
            mapped_address,
            last_screen_state,
            texture,
            canvas,
        }
    }
    pub fn draw_screen(&mut self, nes: &Nes) -> Vec<u8> {
        let mut frame = Vec::with_capacity(32*32*3);
        for i in 0..(32*32) {
            let color = nes.extract_memory(self.mapped_address + i);
            if color == 4 {
                // should be "green"
                frame.push(0);
                frame.push(255);
                frame.push(0);
            } else if color == 3 {
                // should be "red"
                frame.push(255);
                frame.push(0);
                frame.push(0);
            } else {
                // draw background
                frame.push(0);
                frame.push(0);
                frame.push(0);
            }
        }
        frame
    }
}

impl<'a> NesPeripheral for SimpleScreen<'a> {
    fn init(&mut self, _nes: &mut Nes) {}
    fn tick(&mut self, nes: &mut Nes) {
        if memory_changed(
            nes,
            self.mapped_address,
            self.last_screen_state.len() as u16,
            &self.last_screen_state,
        ) {
            let pixel_data = self.draw_screen(nes);
            self.texture
                .update(None, &pixel_data, 32 * 3)
                .unwrap();
            self.canvas.copy(&self.texture, None, None).unwrap();
            self.canvas.present();
        }

        // // init frame
        // for x in 0..32 {
        //     for y in 0..32 {
        //         let x = x - 1;
        //         let y = y - 1;
        //         let color = nes.extract_memory(self.mapped_address + x * 32 + y);
        //         if color == 4 {
        //             // should be "green"
        //         } else if color == 3 {
        //             // should be "red"
        //         } else {
        //             // draw background
        //         }
        //     }
        // }
        // wrap up frame
    }
    fn cleanup(&mut self, _nes: &mut Nes) {}
}

fn memory_changed(nes: &Nes, address: u16, size: u16, old: &[u8]) -> bool {
    for i in 0..size {
        if nes.extract_memory(i + address) != old[i as usize] {
            return false;
        }
    }
    true
}
