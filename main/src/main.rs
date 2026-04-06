mod cpu;

use cpu::Cpu;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCALE: u32 = 10; // Make window larger, each pixel becomes 10x10 window pixels

const KEYMAP: [Keycode; 16] = [
    Keycode::X,
    Keycode::_1,
    Keycode::_2,
    Keycode::_3,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::Z,
    Keycode::C,
    Keycode::_4,
    Keycode::R,
    Keycode::F,
    Keycode::V,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_buffer = parse_args(env::args().collect());

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_buffer)?;

    // ATTEMPT AT VIDEO INITIALIZATION //
    let sdl_context = sdl3::init()?;
    let vid_subsys = sdl_context.video()?;

    let window = vid_subsys
        .window(
            "Chip-8 Emulator",
            SCREEN_WIDTH * SCALE,
            SCREEN_HEIGHT * SCALE,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;

    println!("Successfully read {} bytes from ROM", rom_buffer.len());

    // Temporary execution loop
    while program_loop(&mut canvas, &mut event_pump, &mut cpu) {}

    Ok(())
}

fn program_loop(
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    pump: &mut sdl3::EventPump,
    cpu: &mut Cpu,
) -> bool {
    // Event handlers
    for event in pump.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                if let Some(index) = KEYMAP.iter().position(|&k| k == key) {
                    cpu.keys[index] = true;
                }
                if key == Keycode::Escape {
                    return false;
                }
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                if let Some(index) = KEYMAP.iter().position(|&k| k == key) {
                    cpu.keys[index] = false;
                }
            }
            _ => {}
        }
    }

    // Run CPU instructions
    cpu.step();
    cpu.render(canvas);

    // TODO: Implement accurate CPU timing logic
    ::std::thread::sleep(Duration::from_millis(2));

    true
}

// TODO: make sure args[1] is actually a file path
// TODO: more robust ability to implement multiple args as needed
fn parse_args(args: Vec<String>) -> Vec<u8> {
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let mut file = File::open(&path).expect(&format!("Couldn't open {}", path));
    let mut rom_buffer = Vec::new();

    file.read_to_end(&mut rom_buffer)
        .expect("Failed to read file.\n");

    rom_buffer
}
