use crate::cpu::{Cpu, CpuContext};

impl Cpu {
    pub fn fetch_byte(&mut self, ctx: &mut CpuContext) -> u8 {
        let byte = ctx.bus.read8(self.pc);
        self.pc += 1;

        byte
    }

    pub fn fetch_word(&mut self, ctx: &mut CpuContext) -> u16 {
        let lo = self.fetch_byte(ctx) as u16;
        let hi = self.fetch_byte(ctx) as u16;

        (hi << 8) | lo
    }

    pub fn read_m(&mut self, ctx: &mut CpuContext) -> u8 {
        ctx.bus.read8(self.state.hl())
    }

    pub fn write_m(&mut self, ctx: &mut CpuContext, value: u8) {
        ctx.bus.write8(self.state.hl(), value);
    }
}
