mod audio;
mod cpu;
mod display;
mod framebuffer;

use clap::Parser;
use cpu::Cpu;
use sdl3::{event::Event, keyboard::Keycode};
use std::{
    fs::File,
    io::Read,
    path::Path,
    time::{Duration, Instant},
};

use crate::{display::SDLCreate, framebuffer::FrameBuffer};

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

#[derive(Parser, Debug)]
#[command(name = "ch8")]
#[command(about = "A CHIP-8 emulator")]
struct Args {
    //TODO
    #[arg(long, default_value_t = false)]
    vsync: bool,

    //TODO
    #[arg(long, default_value_t = 50)]
    volume: u8,

    //TODO
    #[arg(short, long, default_value_t = 600)]
    frequency: u32,

    // TODO
    #[arg(short, long, default_value_t = false)]
    log: bool,

    //TODO
    #[arg(long, default_value_t = 800)]
    width: u32,

    //TODO
    #[arg(long, default_value_t = 600)]
    height: u32,

    //TODO
    #[arg(required = true, value_name = "Path to ROM")]
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    //println!("{:?}", args);

    let rom_path = Path::new(&args.rom);
    if !rom_path.exists() {
        eprintln!("ROM path does not exist");
        std::process::exit(1);
    } else if !rom_path.is_file() {
        eprintln!("ROM path is not a file");
        std::process::exit(1);
    }

    let rom_buffer = parse_rom(args.rom);

    let mut sdl_display = SDLCreate::init_display()?;

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_buffer)?;
    println!("Successfully read {} bytes from ROM", rom_buffer.len());

    let mut audio_stream = audio::init_audio_stream(sdl_display.audio_subsystem());
    let mut framebuffer = FrameBuffer::new();

    let mut last_timer_update = Instant::now();
    let mut last_cpu_update = Instant::now();

    while program_loop(
        &mut sdl_display,
        &mut last_timer_update,
        &mut last_cpu_update,
        &mut cpu,
        &mut framebuffer,
        &mut audio_stream,
    ) {}

    Ok(())
}

fn program_loop(
    sdl_display: &mut SDLCreate,
    last_timer_update: &mut Instant,
    last_cpu_update: &mut Instant,
    cpu: &mut Cpu,
    framebuffer: &mut FrameBuffer,
    audio_stream: &mut sdl3::audio::AudioStreamWithCallback<audio::SquareWave>,
) -> bool {
    for event in sdl_display.event_pump().poll_iter() {
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

    let cpu_interval = Duration::from_secs_f64(1.0 / 600.0);
    while last_cpu_update.elapsed() >= cpu_interval {
        cpu.step(framebuffer);
        *last_cpu_update += cpu_interval;
    }

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

    if let Some(mut audio) = audio_stream.lock() {
        audio.set_playing(cpu.soundt_reg > 0);
    }

    sdl_display.render(framebuffer);

    std::thread::sleep(Duration::from_millis(1)); // Help prevent weird user CPU spikes

    true
}

fn parse_rom(arg: String) -> Vec<u8> {
    let mut file = File::open(&arg).expect(&format!("Couldn't open {}", arg));
    let mut rom_buffer = Vec::new();

    file.read_to_end(&mut rom_buffer)
        .expect("Failed to read file.\n");

    rom_buffer
}
