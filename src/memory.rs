pub struct Memory {
    data: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; 0x10000],
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    pub fn load(&mut self, program: &[u8]) {
        self.data[..program.len()].copy_from_slice(program);
    }
}