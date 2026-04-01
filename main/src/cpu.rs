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

    pub fn step(&mut self) {
        let opcode = self.fetch();
        println!("fetched opcode: {:#X}", opcode);
        self.execute(opcode);

        // Bunch of debug stuff, remove later
        println!("executed {:#X}", opcode);
        println!("v_regs:\n{:#?}", self.v_regs);
        println!("stack: {:#?}", self.stack);
        println!("program counter: {:#X}", self.pc_reg);
    }

    fn fetch(&mut self) -> u16 {
        // We take two 8-bit bytes from current position in
        // memory and convert them into a single 16-bit opcode.
        let high = self.memory[self.pc_reg as usize] as u16;
        let low = self.memory[(self.pc_reg + 1) as usize] as u16;
        let opcode = (high << 8) | low;

        self.pc_reg += 2;
        opcode
    }

    fn execute(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => {
                    // Clear display
                    self.display = [0; 64 * 32];
                }
                0x00EE => {
                    // Return
                    if self.sp_reg == 0 {
                        panic!("Stack underflow!");
                    }
                    self.sp_reg -= 1;
                    self.pc_reg = self.stack[self.sp_reg as usize];
                }
                _ => panic!("Encountered unknown opcode! {:#X}", opcode),
            },
            0x1000 => {
                // 1NNN: Jump to address NNN
                let addr = opcode & 0x0FFF;
                self.pc_reg = addr;
            }
            0x2000 => {
                // 2NNN: Call subroutine at NNN and jump
                if self.sp_reg as usize <= self.stack.len() {
                    panic!("Stack overflow!");
                }
                let addr = opcode & 0x0FFF;
                self.sp_reg += 1;
                self.stack[self.sp_reg as usize - 1] = self.pc_reg;
                self.pc_reg = addr;
            }
            0x3000 => {
                // 3XKK: Skip next instruction if Vx = kk
                let (x, kk) = Self::grab_xkk(opcode);
                if self.v_regs[x] == kk {
                    self.pc_reg += 2;
                }
            }
            0x4000 => {
                // 4XKK: Skip next instruction if Vx != kk
                let (x, kk) = Self::grab_xkk(opcode);
                if self.v_regs[x] != kk {
                    self.pc_reg += 2;
                }
            }
            0x5000 => {
                // 5xy0: Skip next instruction if Vx = Vy
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v_regs[x] == self.v_regs[y] {
                    self.pc_reg += 2;
                }
            }
            0x6000 => {
                // 6xkk: Set Vx = kk
                let (x, kk) = Self::grab_xkk(opcode);
                self.v_regs[x] = kk;
            }
            // TODO: don't panic
            _ => panic!("Encountered unimplemented opcode: {:#X}", opcode),
        }
    }

    fn grab_xkk(opcode: u16) -> (usize, u8) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let kk = (opcode & 0x00FF) as u8;
        (x, kk)
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

        // Load ROM into memory starting at 0x200
        self.memory[512..512 + rom_size].copy_from_slice(rom_buffer);
        Ok(())
    }
}
