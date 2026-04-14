mod audio;
mod cpu;
mod display;
mod framebuffer;

use clap::Parser;
use cpu::Cpu;
use sdl2::{audio::AudioDevice, event::Event, keyboard::Keycode};
use std::{
    fs::File,
    io::Read,
    path::Path,
    time::{Duration, Instant},
};

use crate::{display::SDLContext, framebuffer::FrameBuffer};

const KEYMAP: [Keycode; 16] = [
    Keycode::X,
    Keycode::NUM_1,
    Keycode::NUM_2,
    Keycode::NUM_3,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::Z,
    Keycode::C,
    Keycode::NUM_4,
    Keycode::R,
    Keycode::F,
    Keycode::V,
];

#[derive(Parser, Debug)]
#[command(name = "ch8")]
#[command(about = "A CHIP-8 emulator")]
struct Args {
    /*
    //TODO
    /// Enable vertical sync (may help with screen tearing in certain programs)
    #[arg(long, default_value_t = false)]
    vsync: bool,

    //TODO
    /// 0-100
    #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u32).range(0..=100))]
    volume: u8,

    //TODO
    /// Set clock frequency in Hz. 1-1000.
    #[arg(short, long, default_value_t = 600, value_parser = clap::value_parser!(u32).range(1..=1000))]
    frequency: u32,

    // TODO
    /// Enable output logging
    #[arg(short, long, default_value_t = false)]
    log: bool,

    //TODO
    /// Window width in px. 10-1920
    #[arg(long, default_value_t = 800, value_parser = clap::value_parser!(u32).range(10..=1920))]
    width: u32,

    //TODO
    /// Window height in px. 10-1080
    #[arg(long, default_value_t = 600, value_parser = clap::value_parser!(u32).range(1..=1080))]
    height: u32,

    //TODO
    /// Addresses an ambiguous instruction. Try enabling if certain programs aren't behaving quite correctly.
    #[arg(long, default_value_t = false)]
    vy: bool,
    */
    #[arg(required = true, value_name = "Path to ROM")]
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let rom_path = Path::new(&args.rom);
    if !rom_path.exists() {
        eprintln!("ROM path does not exist");
        std::process::exit(1);
    } else if !rom_path.is_file() {
        eprintln!("ROM path is not a file");
        std::process::exit(1);
    }

    let rom_buffer = parse_rom(args.rom);

    let mut sdl_display = SDLContext::new()?;

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_buffer)?;
    println!("Successfully read {} bytes from ROM", rom_buffer.len());

    let mut audio_device = audio::init_audio_device(sdl_display.audio());
    let mut framebuffer = FrameBuffer::new();

    let mut last_timer_update = Instant::now();
    let mut last_cpu_update = Instant::now();

    while program_loop(
        &mut sdl_display,
        &mut last_timer_update,
        &mut last_cpu_update,
        &mut cpu,
        &mut framebuffer,
        &mut audio_device,
    ) {}

    Ok(())
}

fn program_loop(
    sdl_display: &mut SDLContext,
    last_timer_update: &mut Instant,
    last_cpu_update: &mut Instant,
    cpu: &mut Cpu,
    framebuffer: &mut FrameBuffer,
    audio_device: &mut AudioDevice<audio::SquareWave>,
) -> bool {
    for event in sdl_display.events().poll_iter() {
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

    // PROGRAM EXECUTION
    let cpu_interval = Duration::from_secs_f64(1.0 / 600.0); // 600 Hz
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

    // AUDIO
    let mut audio = audio_device.lock();
    audio.set_playing(cpu.soundt_reg > 0);

    // RENDER
    sdl_display.render(framebuffer);
    framebuffer.draw_flag = false;

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
