use crate::memory::Memory;

pub const SCREEN_WIDTH: usize = 224;
pub const SCREEN_HEIGHT: usize = 256;

const VRAM_START: usize = 0x2400;
const VRAM_END: usize = 0x4000;

pub struct Video {
    framebuffer: Vec<u32>,
}

impl Video {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn framebuffer(&self) -> &[u32] {
        &self.framebuffer
    }

    fn vram<'a>(memory: &'a Memory) -> &'a [u8] {
        &memory.data[VRAM_START..VRAM_END]
    }

    pub fn render(&mut self, memory: &Memory) {
        let vram = Self::vram(memory);

        for (i, byte) in vram.iter().enumerate() {
            let x = i / 32;
            let y = (i % 32) * 8;

            for bit in 0..8 {
                let pixel = (byte >> bit) & 1;

                let screen_x = x;
                let screen_y = 255 - (y + bit);

                let index = screen_y * SCREEN_WIDTH + screen_x;

                self.framebuffer[index] =
                    if pixel != 0 {
                        0xFFFFFFFF
                    } else {
                        0x00000000
                    };
            }
        }
    }
}
