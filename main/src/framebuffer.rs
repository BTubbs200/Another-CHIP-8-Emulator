pub struct FrameBuffer {
    pub buffer: [u8; 64 * 32],
    pub draw_flag: bool,
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0; 64 * 32],
            draw_flag: false,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.draw_flag = true;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) -> bool {
        if x >= 64 || y >= 32 {
            return false; // Out of bounds
        }

        let index = y * 64 + x;
        let collision = self.buffer[index] == 1 && value == 1;
        self.buffer[index] ^= value;
        self.draw_flag = true;
        collision
    }
}
