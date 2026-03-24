#[derive(Debug)]
pub struct Cpu {
    memory: [u8; 4096],
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
    fn reset(&mut self) {
        self.memory = [0; 4096];
        self.v_regs = [0; 16];
        self.index_reg = 0;
        self.pc_reg = 0x200;
        self.stack = [0; 16];
        self.sp_reg = 0;
        self.delayt_reg = 0;
        self.soundt_reg = 0;
    }
}
