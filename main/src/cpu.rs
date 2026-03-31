use crate::cpu;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Debug)]
pub struct Cpu {
    memory: [u8; 4096],
    display: [u8; 64 * 32],

    v_regs: [u8; 16], // 16 8-bit V registers 0-F
    index_reg: u16,
    pc_reg: u16,
    stack: [u16; 16],
    sp_reg: u8,
    delayt_reg: u8,
    soundt_reg: u8,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            memory: [0; 4096],
            display: [0; 64 * 32],
            v_regs: [0; 16],
            index_reg: 0,
            pc_reg: 0x200, // Start at address 512
            stack: [0; 16],
            sp_reg: 0,
            delayt_reg: 0,
            soundt_reg: 0,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            ..Default::default()
        };

        // Store fontset into memory starting at 0x50
        cpu.memory[0x50..0x50 + FONTSET.len()].copy_from_slice(&FONTSET);

        cpu
    }

    pub fn load_rom(&mut self, rom_buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let rom_size = rom_buffer.len();

        // TODO: custom error
        if rom_size > (4096 - 512) {
            return Err(format!(
                "ROM too large. Maximum of 3584 bytes, current file is {} bytes",
                rom_size
            )
            .into());
        }

        // Load ROM into memory starting at address 512
        self.memory[512..512 + rom_size].copy_from_slice(rom_buffer);
        Ok(())
    }
}
