use crate::cpu::{Cpu, CpuContext};

impl Cpu {
    pub fn push_word(&mut self, ctx: &mut CpuContext, value: u16) {
        self.sp -= 2;

        ctx.bus.write8(self.sp, value as u8);
        ctx.bus.write8(self.sp + 1, (value >> 8) as u8);
    }

    pub fn pop_word(&mut self, ctx: &mut CpuContext) -> u16 {
        let lo = ctx.bus.read8(self.sp) as u16;
        let hi = ctx.bus.read8(self.sp + 1) as u16;
        self.sp += 2;

        (hi << 8) | lo
    }
}
