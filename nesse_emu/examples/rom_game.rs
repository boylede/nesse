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

const SNAKE_ROM: &[u8] = include_bytes!("snake.nes");

/// list of labels within the game code,
/// for debugging purposes
const LABEL_LIST: &[(u16, &str)] = &[
    (0x0600, "start"),
    (0x0606, "init"),
    (0x060d, "init_snake"),
    (0x062a, "generate_apple_position"),
    (0x0638, "loop"),
    (0x064d, "readkeys"),
    (0x0660, "up_key"),
    (0x066b, "right_key"),
    (0x0676, "down_key"),
    (0x0681, "left_key"),
    (0x068c, "illegal_move"),
    (0x068d, "check_collision"),
    (0x0694, "check_apple_collision"),
    (0x06a8, "check_snake_collision"),
    (0x06c3, "updateSnake"),
    (0x0000, "___"),
    (0x0000, "___"),
    (0x0000, "___"),
    (0x0000, "___"),
    (0x0000, "___"),
    (0x0000, "___"),
    (0x0000, "___"),
    (0x0000, "___"),
    (0x0000, "___"),
    (0x72d, "unknown"),
];

fn main() {
    let snake_cartridge = NesCart::from_slice(&SNAKE_ROM).expect("constant rom failed to load");
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem
        .window("Snake game", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let event_pump = context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let creator = canvas.texture_creator();
    let texture = creator
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
        
        .with_peripheral(&mut screen)
        // .with_peripheral(&mut rate)
        // .with_peripheral(Box::new(PCPrinter))
        .with_peripheral(&mut spy)
        // .with_initial_memory(0x600, &GAME_CODE);
        ;
    nes.insert_cartridge(snake_cartridge);
    nes.init();

    nes.run_until_nop();
    nes.cleanup();
    // loop{}
}

pub struct RateLimiter;

impl NesPeripheral for RateLimiter {
    fn tick(&mut self, nes: &mut Nes) {
        ::std::thread::sleep(std::time::Duration::new(0, 1)); // 16_666_666
    }
}

pub struct Spy;

impl NesPeripheral for Spy {
    fn tick(&mut self, nes: &mut Nes) {
        let regs = nes.dump_registers();
        let next_opcode = nes.peek_pc();
        let stack = nes.dump_stack();
        let pc = regs.get_pc();
        if let Some((_, label)) = LABEL_LIST.iter().find(|(address, _)| *address == pc) {
            println!("LABEL {} at {:x}", label, pc);
        }
        // print!("{:2x} ## {:?} ", next_opcode, regs);
        // println!("{}", stack);
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
            // println!("{:?}", event);
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    println!("pressed w, writing {} to {}", 0x77, self.0);
                    nes.inject_memory_value(self.0, 0x77);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    println!("pressed s, writing {} to {}", 0x73, self.0);
                    nes.inject_memory_value(self.0, 0x73);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    println!("pressed a, writing {} to {}", 0x61, self.0);
                    nes.inject_memory_value(self.0, 0x61);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    println!("pressed d, writing {} to {}", 0x64, self.0);
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
    pub fn update_window(&mut self, nes: &Nes) {
        let pixel_data = self.draw_frame(nes);
        self.texture.update(None, &pixel_data, 32 * 3).unwrap();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
    }
    pub fn draw_frame(&mut self, nes: &Nes) -> Vec<u8> {
        let mut frame = Vec::with_capacity(32 * 32 * 3);
        let buffer = nes.extract_memory_region(self.mapped_address, 32 * 32);
        for color in buffer.iter() {
            let pixel = match color {
                0 => Color::BLACK,
                1 => Color::WHITE,
                2 | 9 => Color::GREY,
                3 | 10 => Color::RED,
                4 | 11 => Color::GREEN,
                5 | 12 => Color::BLUE,
                6 | 13 => Color::MAGENTA,
                7 | 14 => Color::YELLOW,
                _ => Color::CYAN,
            };
            let (r, g, b) = pixel.rgb();
            frame.push(r);
            frame.push(g);
            frame.push(b);
        }
        self.last_screen_state = buffer;
        frame
    }
}

impl<'a> NesPeripheral for SimpleScreen<'a> {
    fn init(&mut self, nes: &mut Nes) {
        self.update_window(nes);
    }
    fn tick(&mut self, nes: &mut Nes) {
        if memory_changed(
            nes,
            self.mapped_address,
            self.last_screen_state.len() as u16,
            &self.last_screen_state,
        ) {
            self.update_window(nes);
        }
    }
    fn cleanup(&mut self, _nes: &mut Nes) {}
}

fn memory_changed(nes: &Nes, address: u16, size: u16, old: &[u8]) -> bool {
    for i in 0..size {
        if nes.extract_memory(i + address) != old[i as usize] {
            println!("screen updated");
            return true;
        }
    }
    false
}
