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
#[command(about = "A CHIP-8 emulator written in Rust")]
struct Args {
    /*
    //TODO
    /// 0-100
    #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u32).range(0..=100))]
    volume: u8,
    */
    /// Verbosity: 0=info, 1=debug, 2=trace
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[arg(short, long)]
    quiet: bool,

    /// Address an ambiguous program instruction. Try disabling if a program isn't behaving correctly. [default: enabled]
    #[arg(long, default_value_t = true)]
    vy: bool,

    /// Enable vertical sync [default: off]
    #[arg(long, default_value_t = false)]
    vsync: bool,

    /// Set clock frequency in Hz. 1-1500.
    #[arg(short, long, default_value_t = 600, value_parser = clap::value_parser!(u32).range(1..=1500))]
    frequency: u32,

    /// Specify scaling for the 64x32 emulation window
    #[arg(long, default_value_t = 10, value_parser = clap::value_parser!(u32).range(1..=30))]
    scale: u32,

    #[arg(required = true, value_name = "Path to ROM")]
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let log_level = if args.quiet {
        "error"
    } else {
        match args.verbose {
            0 => "info",
            1 => "debug",
            _ => "trace",
        }
    };

    // Initialize env_logger, but let RUST_LOG override if set
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    log::debug!(
        "Initializing emulator with following parameters: {:?}",
        args,
    );

    let rom_path = Path::new(&args.rom);
    if !rom_path.exists() {
        log::error!("ROM path does not exist.");
        std::process::exit(1);
    } else if !rom_path.is_file() {
        log::error!("ROM path is not a file.");
        std::process::exit(1);
    }

    log::info!("Emulator starting...");

    let rom_buffer = parse_rom(&args.rom);
    log::info!("ROM loaded successfully: {} bytes", rom_buffer.len());

    let mut sdl_display = SDLContext::new(args.scale, args.vsync)?;
    log::info!(
        "Initialized emulation window of size {}x{}",
        64 * args.scale,
        32 * args.scale
    );

    if args.frequency < 1000 {
        log::info!("Emulator running at frequency of {} Hz", args.frequency)
    } else {
        log::info!(
            "Emulator running at frequency of {} kHz",
            (args.frequency as f32 / 1000.0)
        )
    }

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_buffer)?;

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
        &args,
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
    args: &Args,
) -> bool {
    // EVENT HANDLING
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
    let cpu_interval = Duration::from_secs_f64(1.0 / args.frequency as f64);
    while last_cpu_update.elapsed() >= cpu_interval {
        cpu.step(framebuffer, args.vy);
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

    std::thread::sleep(Duration::from_millis(1)); // Help manage CPU usage

    true
}

fn parse_rom(arg: &str) -> Vec<u8> {
    let mut file = File::open(arg).expect("Couldn't open ROM file.\n");
    let mut rom_buffer = Vec::new();

    file.read_to_end(&mut rom_buffer)
        .expect("Failed to read ROM file.\n");

    rom_buffer
}
