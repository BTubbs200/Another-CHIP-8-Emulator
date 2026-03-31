mod cpu;

use cpu::Cpu;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_buffer = parse_args(env::args().collect());

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_buffer)?;

    println!(
        "Read {} bytes\nROM Contents:\n{:?}",
        rom_buffer.len(),
        rom_buffer
    );
    println!("CPU: {:#?}", cpu);

    Ok(())
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
