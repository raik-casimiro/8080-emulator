use crate::cpu::{Cpu, CpuContext};

impl Cpu {
    pub fn interrupt(&mut self, ctx: &mut CpuContext, interrupt_num: u8) {
        if !self.enable_interrupts {
            return;
        }

        self.enable_interrupts = false;
        self.push_word(ctx, self.pc);
        self.pc = (interrupt_num as u16) * 8;
    }
}
