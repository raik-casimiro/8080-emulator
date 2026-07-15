pub struct ShiftRegister {
    value: u16,
    offset: u8,
}

impl ShiftRegister {
    pub fn new() -> Self {
        Self {
            value: 0,
            offset: 0,
        }
    }

    pub fn write(&mut self, value: u8) {
        self.value >>= 8;
        self.value |= (value as u16) << 8;
    }

    pub fn set_offset(&mut self, offset: u8) {
        self.offset = offset & 0x07;
    }

    pub fn read(&self) -> u8 {
        ((self.value >> (8 - self.offset)) & 0xFF) as u8
    }
}
