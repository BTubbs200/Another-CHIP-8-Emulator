mod cpu;

use cpu::Cpu;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // Parse args, expect ROM file to be provided
    let buffer = parse_args(env::args().collect());
    let cpu: Cpu = Default::default();

    // TODO: actually do real stuff with ROM file
    println!("Read {} bytes\nContents: {:?}", buffer.len(), buffer);
    println!("CPU: {:#?}", cpu);
}

fn parse_args(args: Vec<String>) -> Vec<u8> {
    if args.len() != 2 {
        eprintln!(
            "Path to file not specified.\nUsage: {} <file_path>",
            args[0]
        );
        std::process::exit(1);
    }

    let path = &args[1];
    let mut file = File::open(&path).expect(&format!("Couldn't open {}", path));
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .expect("Failed to read file.\n");

    return buffer;
}
