use crate::Bus;
use crate::cpu::{Cpu, CpuContext};

impl Cpu {
    pub fn add(&mut self, value: u8) {
        self.state.flags.update_auxiliary_carry_add(self.state.a, value);

        let (result, overflow) = self.state.a.overflowing_add(value);
        self.state.flags.update_szp(result);
        self.state.flags.carry = overflow;

        self.state.a = result;
    }

    pub fn adc(&mut self, value: u8) {
        let old_carry = self.state.flags.carry as u8;

        self.state.flags.auxiliary_carry = ((self.state.a & 0x0F) + (value & 0x0F) + old_carry) > 0x0F;

        let (result, carry1) = self.state.a.overflowing_add(value);
        let (result, carry2) = result.overflowing_add(old_carry);

        self.state.a = result;
        self.state.flags.update_szp(result);
        self.state.flags.carry = carry1 || carry2;
    }

    pub fn sub(&mut self, value: u8) {
        self.state.flags.update_auxiliary_carry_sub(self.state.a, value);

        let (result, borrow) = self.state.a.overflowing_sub(value);

        self.state.a = result;

        self.state.flags.carry = borrow;
        self.state.flags.update_szp(result);
    }

    pub fn sbb(&mut self, value: u8) {
        let old_carry = self.state.flags.carry as u8;

        self.state.flags.auxiliary_carry = (self.state.a & 0x0F) < ((value & 0x0F) + old_carry);

        let (result, borrow1) = self.state.a.overflowing_sub(value);
        let (result, borrow2) = result.overflowing_sub(old_carry);

        self.state.a = result;
        self.state.flags.carry = borrow1 || borrow2;
        self.state.flags.update_szp(result);
    }

    pub fn ana(&mut self, value: u8) {
        self.state.flags.auxiliary_carry = true;
        self.state.flags.carry = false;

        self.state.a &= value;
        self.state.flags.update_szp(self.state.a);
    }

    pub fn xra(&mut self, value: u8) {
        self.state.flags.auxiliary_carry = false;
        self.state.flags.carry = false;

        self.state.a ^= value;
        self.state.flags.update_szp(self.state.a);
    }

    pub fn ora(&mut self, value: u8) {
        self.state.flags.auxiliary_carry = false;
        self.state.flags.carry = false;

        self.state.a |= value;
        self.state.flags.update_szp(self.state.a);
    }

    pub fn cmp(&mut self, value: u8) {
        self.state.flags.update_auxiliary_carry_sub(self.state.a, value);

        let (result, borrow) = self.state.a.overflowing_sub(value);
        self.state.flags.carry = borrow;
        self.state.flags.update_szp(result);
    }

    pub fn rst(&mut self, ctx: &mut CpuContext, address: u16) {
        self.push_word(ctx, self.pc);
        self.pc = address;
    }
}
