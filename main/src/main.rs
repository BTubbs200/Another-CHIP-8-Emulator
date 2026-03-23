use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // Parse args, expect ROM file to be provided
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let mut file = File::open(&path).expect(&format!("Couldn't open {}", path));
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .expect("Failed to read file.\n");

    // TODO: actually do real stuff with ROM file
    println!("Read {} bytes", buffer.len());
}
