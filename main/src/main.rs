mod audio;
mod cpu;

use cpu::Cpu;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

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

    let sdl_context = sdl3::init()?;
    let vid_subsys = sdl_context.video()?;
    let audio_subsys = sdl_context.audio().unwrap();

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

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_buffer)?;
    println!("Successfully read {} bytes from ROM", rom_buffer.len());

    let mut audio_stream = audio::init_audio_stream(&audio_subsys);

    let mut last_timer_update = Instant::now();
    let mut last_cpu_update = Instant::now();

    while program_loop(
        &mut canvas,
        &mut event_pump,
        &mut last_timer_update,
        &mut last_cpu_update,
        &mut cpu,
        &mut audio_stream,
    ) {}

    Ok(())
}

fn program_loop(
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    pump: &mut sdl3::EventPump,
    last_timer_update: &mut Instant,
    last_cpu_update: &mut Instant,
    cpu: &mut Cpu,
    audio_stream: &mut sdl3::audio::AudioStreamWithCallback<audio::SquareWave>,
) -> bool {
    // Event handlers
    for event in pump.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                if let Some(index) = KEYMAP.iter().position(|&k| k == key) {
                    if !cpu.keys[index] {
                        cpu.on_key_press(index as u8);
                    }
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
                    cpu.on_key_release(index as u8);
                }
            }
            _ => {}
        }
    }

    // CPU (600Hz)
    let cpu_interval = Duration::from_secs_f64(1.0 / 600.0);
    while last_cpu_update.elapsed() >= cpu_interval {
        // Run CPU instructions
        cpu.step();
        *last_cpu_update += cpu_interval;
    }

    // TIMERS (60 Hz)
    let timer_interval = Duration::from_secs_f64(1.0 / 60.0);
    while last_timer_update.elapsed() >= timer_interval {
        if cpu.delayt_reg > 0 {
            cpu.delayt_reg -= 1;
        }

        if cpu.soundt_reg > 0 {
            cpu.soundt_reg -= 1;
        }

        *last_timer_update += timer_interval;
    }

    // AUDIO
    if let Some(mut audio) = audio_stream.lock() {
        audio.set_playing(cpu.soundt_reg > 0);
    }

    cpu.render(canvas);

    std::thread::sleep(Duration::from_millis(1)); // Prevent weird user CPU usage

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
