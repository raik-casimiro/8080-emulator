use crate::memory::Memory;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 224;

const FRAMEBUFFER_START: usize = 0x2400;
const FRAMEBUFFER_END: usize = 0x4000;

pub struct Video;

impl Video {
    pub fn framebuffer(memory: &Memory) -> &[u8] {
        &memory.data[FRAMEBUFFER_START..FRAMEBUFFER_END]
    }
}
