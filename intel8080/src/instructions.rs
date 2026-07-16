use crate::cpu::{Cpu, CpuContext};

pub fn nop(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    4
}

pub fn lxi_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_word(ctx);

    cpu.state.set_bc(value);
    10
}

pub fn stax_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr: u16 = cpu.state.bc();

    ctx.bus.write8(addr, cpu.state.a);
    7
}

pub fn inx_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let bc: u16 = cpu.state.bc().wrapping_add(1);
    cpu.state.set_bc(bc);
    5
}

pub fn inr_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.b, 1);

    cpu.state.b = cpu.state.b.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.b);
    5
}

pub fn dcr_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.b, 1);

    cpu.state.b = cpu.state.b.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.b);
    5
}

pub fn mvi_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.b = cpu.fetch_byte(ctx);
    7
}

pub fn rlc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.carry = (cpu.state.a >> 7) == 1;
    cpu.state.a = cpu.state.a << 1;
    cpu.state.a = cpu.state.a | (cpu.state.flags.carry as u8);
    7
}

pub fn dad_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let bc: u16 = cpu.state.bc();
    let hl: u16 = cpu.state.hl();
    let (result, overflow) = hl.overflowing_add(bc);

    cpu.state.flags.carry = overflow;

    cpu.state.set_hl(result);
    10
}

pub fn ldax_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr: u16 = cpu.state.bc();
    cpu.state.a = ctx.bus.read8(addr);
    7
}

pub fn dcx_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let bc: u16 = cpu.state.bc().wrapping_sub(1);
    cpu.state.set_bc(bc);
    5
}

pub fn inr_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.c, 1);

    cpu.state.c = cpu.state.c.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.c);
    5
}

pub fn dcr_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.c, 1);

    cpu.state.c = cpu.state.c.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.c);
    5
}

pub fn mvi_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.c = cpu.fetch_byte(ctx);
    7
}

pub fn rrc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let bit = cpu.state.a & 1;

    cpu.state.flags.carry = bit != 0;

    cpu.state.a >>= 1;
    cpu.state.a |= bit << 7;
    4
}

pub fn lxi_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_word(ctx);
    cpu.state.set_de(value);
    10
}

pub fn stax_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr: u16 = cpu.state.de();
    ctx.bus.write8(addr, cpu.state.a);
    7
}

pub fn inx_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let de: u16 = cpu.state.de().wrapping_add(1);
    cpu.state.set_de(de);
    5
}

pub fn inr_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.d, 1);

    cpu.state.d = cpu.state.d.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.d);
    5
}

pub fn dcr_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.d, 1);

    cpu.state.d = cpu.state.d.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.d);
    5
}

pub fn mvi_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.d = cpu.fetch_byte(ctx);
    7
}

pub fn ral(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let old_carry = cpu.state.flags.carry;

    cpu.state.flags.carry = (cpu.state.a >> 7) == 1;
    cpu.state.a = cpu.state.a << 1;
    cpu.state.a = cpu.state.a | (old_carry as u8);
    4
}

pub fn dad_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let de: u16 = cpu.state.de();
    let hl: u16 = cpu.state.hl();
    let (result, overflow) = hl.overflowing_add(de);

    cpu.state.flags.carry = overflow;
    cpu.state.set_hl(result);
    10
}

pub fn ldax_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr: u16 = cpu.state.de();
    cpu.state.a = ctx.bus.read8(addr);
    7
}

pub fn dcx_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let de: u16 = cpu.state.de().wrapping_sub(1);
    cpu.state.set_de(de);
    5
}

pub fn inr_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.e, 1);

    cpu.state.e = cpu.state.e.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.e);
    5
}

pub fn dcr_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.e, 1);

    cpu.state.e = cpu.state.e.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.e);
    5
}

pub fn mvi_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.e = cpu.fetch_byte(ctx);
    7
}

pub fn rar(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let old_carry = cpu.state.flags.carry;

    cpu.state.flags.carry = (cpu.state.a & 1) != 0;

    cpu.state.a >>= 1;

    if old_carry {
        cpu.state.a |= 0x80;
    }
    4
}

pub fn lxi_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_word(ctx);
    cpu.state.set_hl(value);
    10
}

pub fn shld(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);

    ctx.bus.write8(addr, cpu.state.l);
    ctx.bus.write8(addr + 1, cpu.state.h);
    16
}

pub fn inx_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let hl: u16 = cpu.state.hl().wrapping_add(1);
    cpu.state.set_hl(hl);
    5
}

pub fn inr_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.h, 1);

    cpu.state.h = cpu.state.h.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.h);
    5
}

pub fn dcr_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.h, 1);

    cpu.state.h = cpu.state.h.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.h);
    5
}

pub fn mvi_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.h = cpu.fetch_byte(ctx);
    7
}

pub fn daa(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    println!("DAA not implemented");
    4
}

pub fn dad_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let hl: u16 = cpu.state.hl();
    let (result, overflow) = hl.overflowing_add(hl);

    cpu.state.flags.carry = overflow;

    cpu.state.set_hl(result);
    10
}

pub fn lhld(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);

    let lo = ctx.bus.read8(addr);
    let hi = ctx.bus.read8(addr + 1);

    cpu.state.l = lo;
    cpu.state.h = hi;
    16
}

pub fn dcx_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let hl: u16 = cpu.state.hl().wrapping_sub(1);
    cpu.state.set_hl(hl);
    5
}

pub fn inr_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.l, 1);

    cpu.state.l = cpu.state.l.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.l);
    5
}

pub fn dcr_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.l, 1);

    cpu.state.l = cpu.state.l.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.l);
    5
}

pub fn mvi_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.l = cpu.fetch_byte(ctx);
    7
}

pub fn cma(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.a = !cpu.state.a;
    4
}

pub fn lxi_sp(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let data = cpu.fetch_word(ctx);
    cpu.sp = data;
    10
}

pub fn sta(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    ctx.bus.write8(addr, cpu.state.a);
    13
}

pub fn inx_sp(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sp = cpu.sp.wrapping_add(1);
    5
}

pub fn inr_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);

    cpu.state.flags.update_auxiliary_carry_add(value, 1);

    let new_value = value.wrapping_add(1);

    cpu.write_m(ctx, new_value);
    cpu.state.flags.update_szp(new_value);
    10
}

pub fn dcr_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);

    cpu.state.flags.update_auxiliary_carry_sub(value, 1);

    let new_value = value.wrapping_sub(1);

    cpu.write_m(ctx, new_value);
    cpu.state.flags.update_szp(new_value);
    10
}

pub fn mvi_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_byte(ctx);
    cpu.write_m(ctx, value);
    10
}

pub fn stc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.carry = true;
    4
}

pub fn dad_sp(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let hl: u16 = cpu.state.hl();
    let (result, overflow) = hl.overflowing_add(cpu.sp);

    cpu.state.flags.carry = overflow;

    cpu.state.set_hl(result);
    10
}

pub fn lda(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.state.a = ctx.bus.read8(addr);
    13
}

pub fn dcx_sp(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sp = cpu.sp.wrapping_sub(1);
    5
}

pub fn inr_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.a, 1);

    cpu.state.a = cpu.state.a.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.a);
    5
}

pub fn dcr_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.a, 1);

    cpu.state.a = cpu.state.a.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.a);
    5
}

pub fn mvi_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.a = cpu.fetch_byte(ctx);
    7
}

pub fn cmc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.flags.carry = !cpu.state.flags.carry;
    4
}

pub fn mov_bb(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.b = cpu.state.b;
    5
}

pub fn mov_bc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.b = cpu.state.c;
    5
}

pub fn mov_bd(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.b = cpu.state.d;
    5
}

pub fn mov_be(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.b = cpu.state.e;
    5
}

pub fn mov_bh(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.b = cpu.state.h;
    5
}

pub fn mov_bl(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.b = cpu.state.l;
    5
}

pub fn mov_bm(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.state.b = value;
    7
}

pub fn mov_ba(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.b = cpu.state.a;
    5
}

pub fn mov_cb(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.c = cpu.state.b;
    5
}

pub fn mov_cc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.c = cpu.state.c;
    5
}

pub fn mov_cd(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.c = cpu.state.d;
    5
}

pub fn mov_ce(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.c = cpu.state.e;
    5
}

pub fn mov_ch(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.c = cpu.state.h;
    5
}

pub fn mov_cl(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.c = cpu.state.l;
    5
}

pub fn mov_cm(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.state.c = value;
    7
}

pub fn mov_ca(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.c = cpu.state.a;
    5
}

pub fn mov_db(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.d = cpu.state.b;
    5
}

pub fn mov_dc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.d = cpu.state.c;
    5
}

pub fn mov_dd(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.d = cpu.state.d;
    5
}

pub fn mov_de(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.d = cpu.state.e;
    5
}

pub fn mov_dh(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.d = cpu.state.h;
    5
}

pub fn mov_dl(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.d = cpu.state.l;
    5
}

pub fn mov_dm(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value= cpu.read_m(ctx);
    cpu.state.d = value;
    7
}

pub fn mov_da(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.d = cpu.state.a;
    5
}

pub fn mov_eb(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.e = cpu.state.b;
    5
}

pub fn mov_ec(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.e = cpu.state.c;
    5
}

pub fn mov_ed(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.e = cpu.state.d;
    5
}

pub fn mov_ee(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.e = cpu.state.e;
    5
}

pub fn mov_eh(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.e = cpu.state.h;
    5
}

pub fn mov_el(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.e = cpu.state.l;
    5
}

pub fn mov_em(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value= cpu.read_m(ctx);
    cpu.state.e = value;
    7
}

pub fn mov_ea(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.e = cpu.state.a;
    5
}

pub fn mov_hb(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.h = cpu.state.b;
    5
}

pub fn mov_hc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.h = cpu.state.c;
    5
}

pub fn mov_hd(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.h = cpu.state.d;
    5
}

pub fn mov_he(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.h = cpu.state.e;
    5
}

pub fn mov_hh(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.h = cpu.state.h;
    5
}

pub fn mov_hl(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.h = cpu.state.l;
    5
}

pub fn mov_hm(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value= cpu.read_m(ctx);
    cpu.state.h = value;
    7
}

pub fn mov_ha(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.h = cpu.state.a;
    5
}

pub fn mov_lb(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.l = cpu.state.b;
    5
}

pub fn mov_lc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.l = cpu.state.c;
    5
}

pub fn mov_ld(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.l = cpu.state.d;
    5
}

pub fn mov_le(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.l = cpu.state.e;
    5
}

pub fn mov_lh(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.l = cpu.state.h;
    5
}

pub fn mov_ll(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.l = cpu.state.l;
    5
}

pub fn mov_lm(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value= cpu.read_m(ctx);
    cpu.state.l = value;
    7
}

pub fn mov_la(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.l = cpu.state.a;
    5
}

pub fn mov_mb(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.write_m(ctx, cpu.state.b);
    7
}

pub fn mov_mc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.write_m(ctx, cpu.state.c);
    7
}

pub fn mov_md(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.write_m(ctx, cpu.state.d);
    7
}

pub fn mov_me(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.write_m(ctx, cpu.state.e);
    7
}

pub fn mov_mh(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.write_m(ctx, cpu.state.h);
    7
}

pub fn mov_ml(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.write_m(ctx, cpu.state.l);
    7
}

pub fn hlt(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.halted = true;
    7
}

pub fn mov_ma(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.write_m(ctx, cpu.state.a);
    7
}

pub fn mov_ab(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.a = cpu.state.b;
    5
}

pub fn mov_ac(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.a = cpu.state.c;
    5
}

pub fn mov_ad(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.a = cpu.state.d;
    5
}

pub fn mov_ae(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.a = cpu.state.e;
    5
}

pub fn mov_ah(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.a = cpu.state.h;
    5
}

pub fn mov_al(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.a = cpu.state.l;
    5
}

pub fn mov_am(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.state.a = value;
    7
}

pub fn mov_aa(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.state.a = cpu.state.a;
    5
}

pub fn add_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.add(cpu.state.b);
    4
}

pub fn add_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.add(cpu.state.c);
    4
}

pub fn add_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.add(cpu.state.d);
    4
}

pub fn add_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.add(cpu.state.e);
    4
}

pub fn add_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.add(cpu.state.h);
    4
}

pub fn add_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.add(cpu.state.l);
    4
}

pub fn add_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.add(value);
    7
}

pub fn add_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.add(cpu.state.a);
    4
}

pub fn adc_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.adc(cpu.state.b);
    4
}

pub fn adc_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.adc(cpu.state.c);
    4
}

pub fn adc_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.adc(cpu.state.d);
    4
}

pub fn adc_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.adc(cpu.state.e);
    4
}

pub fn adc_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.adc(cpu.state.h);
    4
}

pub fn adc_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.adc(cpu.state.l);
    4
}

pub fn adc_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.adc(value);
    7
}

pub fn adc_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.adc(cpu.state.a);
    4
}

pub fn sub_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sub(cpu.state.b);
    4
}

pub fn sub_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sub(cpu.state.c);
    4
}

pub fn sub_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sub(cpu.state.d);
    4
}

pub fn sub_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sub(cpu.state.e);
    4
}

pub fn sub_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sub(cpu.state.h);
    4
}

pub fn sub_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sub(cpu.state.l);
    4
}

pub fn sub_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.sub(value);
    7
}

pub fn sub_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sub(cpu.state.a);
    4
}

pub fn sbb_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sbb(cpu.state.b);
    4
}

pub fn sbb_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sbb(cpu.state.c);
    4
}

pub fn sbb_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sbb(cpu.state.d);
    4
}

pub fn sbb_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sbb(cpu.state.e);
    4
}

pub fn sbb_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sbb(cpu.state.h);
    4
}

pub fn sbb_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sbb(cpu.state.l);
    4
}

pub fn sbb_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.sbb(value);
    7
}

pub fn sbb_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sbb(cpu.state.a);
    4
}

pub fn ana_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ana(cpu.state.b);
    4
}

pub fn ana_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ana(cpu.state.c);
    4
}

pub fn ana_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ana(cpu.state.d);
    4
}

pub fn ana_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ana(cpu.state.e);
    4
}

pub fn ana_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ana(cpu.state.h);
    4
}

pub fn ana_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ana(cpu.state.l);
    4
}

pub fn ana_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.ana(value);
    7
}

pub fn ana_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ana(cpu.state.a);
    4
}

pub fn xra_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.xra(cpu.state.b);
    4
}

pub fn xra_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.xra(cpu.state.c);
    4
}

pub fn xra_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.xra(cpu.state.d);
    4
}

pub fn xra_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.xra(cpu.state.e);
    4
}

pub fn xra_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.xra(cpu.state.h);
    4
}

pub fn xra_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.xra(value);
    7
}

pub fn xra_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.xra(cpu.state.l);
    4
}

pub fn xra_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.xra(cpu.state.a);
    4
}

pub fn ora_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ora(cpu.state.b);
    4
}

pub fn ora_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ora(cpu.state.c);
    4
}

pub fn ora_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ora(cpu.state.d);
    4
}

pub fn ora_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ora(cpu.state.e);
    4
}

pub fn ora_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ora(cpu.state.h);
    4
}

pub fn ora_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ora(cpu.state.l);
    4
}

pub fn ora_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.ora(value);
    7
}

pub fn ora_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.ora(cpu.state.a);
    4
}

pub fn cmp_b(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.cmp(cpu.state.b);
    4
}

pub fn cmp_c(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.cmp(cpu.state.c);
    4
}

pub fn cmp_d(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.cmp(cpu.state.d);
    4
}

pub fn cmp_e(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.cmp(cpu.state.e);
    4
}

pub fn cmp_h(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.cmp(cpu.state.h);
    4
}

pub fn cmp_l(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.cmp(cpu.state.l);
    4
}

pub fn cmp_m(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.read_m(ctx);
    cpu.cmp(value);
    7
}

pub fn cmp_a(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.cmp(cpu.state.a);
    4
}

// Conditional returns: 11 cycles if taken, 5 if not
pub fn rnz(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    if cpu.return_if(ctx, !cpu.state.flags.zero) { 11 } else { 5 }
}

pub fn pop_bc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.pop_word(ctx);
    cpu.state.set_bc(value);
    10
}

// Conditional jumps: 10 cycles always
pub fn jnz(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.jump_if(addr, !cpu.state.flags.zero);
    10
}

pub fn jmp(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.pc = addr;
    10
}

// Conditional calls: 17 cycles if taken, 11 if not
pub fn cnz(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    if cpu.call_if(ctx, addr, !cpu.state.flags.zero) { 17 } else { 11 }
}

pub fn push_bc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.push_word(ctx, cpu.state.bc());
    11
}

pub fn adi(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_byte(ctx);
    cpu.add(value);
    7
}

pub fn rst_0(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.rst(ctx, 0x0000);
    11
}

pub fn rz(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    if cpu.return_if(ctx, cpu.state.flags.zero) { 11 } else { 5 }
}

pub fn ret(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.pc = cpu.pop_word(ctx);
    10
}

pub fn jz(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.jump_if(addr, cpu.state.flags.zero);
    10
}

pub fn cz(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    if cpu.call_if(ctx, addr, cpu.state.flags.zero) { 17 } else { 11 }
}

pub fn call(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.call_if(ctx, addr, true);
    17
}

pub fn aci(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_byte(ctx);
    cpu.adc(value);
    7
}

pub fn rst_1(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.rst(ctx, 0x0008);
    11
}

pub fn rnc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    if cpu.return_if(ctx, !cpu.state.flags.carry) { 11 } else { 5 }
}

pub fn pop_de(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.pop_word(ctx);
    cpu.state.set_de(value);
    10
}

pub fn jnc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.jump_if(addr, !cpu.state.flags.carry);
    10
}

pub fn out(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let port = cpu.fetch_byte(ctx);
    ctx.bus.output(port, cpu.state.a);
    10
}

pub fn cnc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    if cpu.call_if(ctx, addr, !cpu.state.flags.carry) { 17 } else { 11 }
}

pub fn push_de(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.push_word(ctx, cpu.state.de());
    11
}

pub fn sui(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_byte(ctx);
    cpu.sub(value);
    7
}

pub fn rst_2(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.rst(ctx, 0x0010);
    11
}

pub fn rc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    if cpu.return_if(ctx, cpu.state.flags.carry) { 11 } else { 5 }
}

pub fn jc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.jump_if(addr, cpu.state.flags.carry);
    10
}

pub fn in_(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let port = cpu.fetch_byte(ctx);
    cpu.state.a = ctx.bus.input(port);
    10
}

pub fn cc(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    if cpu.call_if(ctx, addr, cpu.state.flags.carry) { 17 } else { 11 }
}

pub fn sbi(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_byte(ctx);
    cpu.sbb(value);
    7
}

pub fn rst_3(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.rst(ctx, 0x0018);
    11
}

pub fn rpo(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    if cpu.return_if(ctx, !cpu.state.flags.parity) { 11 } else { 5 }
}

pub fn pop_hl(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.pop_word(ctx);
    cpu.state.set_hl(value);
    10
}

pub fn jpo(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.jump_if(addr, !cpu.state.flags.parity);
    10
}

pub fn xthl(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let stack_value = cpu.pop_word(ctx);
    let hl = cpu.state.hl();

    cpu.push_word(ctx, hl);
    cpu.state.set_hl(stack_value);
    18
}

pub fn cpo(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    if cpu.call_if(ctx, addr, !cpu.state.flags.parity) { 17 } else { 11 }
}

pub fn push_hl(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.push_word(ctx, cpu.state.hl());
    11
}

pub fn ani(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_byte(ctx);
    cpu.ana(value);
    7
}

pub fn rst_4(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.rst(ctx, 0x0020);
    11
}

pub fn rpe(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    if cpu.return_if(ctx, cpu.state.flags.parity) { 11 } else { 5 }
}

pub fn pchl(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let hl = cpu.state.hl();
    cpu.pc = hl;
    5
}

pub fn jpe(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.jump_if(addr, cpu.state.flags.parity);
    10
}

pub fn xchg(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let hl = cpu.state.hl();
    let de = cpu.state.de();

    cpu.state.set_hl(de);
    cpu.state.set_de(hl);
    4
}

pub fn cpe(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    if cpu.call_if(ctx, addr, cpu.state.flags.parity) { 17 } else { 11 }
}

pub fn xri(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_byte(ctx);
    cpu.xra(value);
    7
}

pub fn rst_5(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.rst(ctx, 0x0028);
    11
}

pub fn rp(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    if cpu.return_if(ctx, cpu.state.flags.sign) { 11 } else { 5 }
}

pub fn pop_psw(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let psw = cpu.pop_word(ctx);

    cpu.state.flags.unpack(psw as u8);
    cpu.state.a = (psw >> 8) as u8;
    10
}

pub fn jp(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.jump_if(addr, cpu.state.flags.sign);
    10
}

pub fn di(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.enable_interrupts = false;
    4
}

pub fn cp(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    if cpu.call_if(ctx, addr, cpu.state.flags.sign) { 17 } else { 11 }
}

pub fn push_psw(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let psw = ((cpu.state.a as u16) << 8) | cpu.state.flags.pack() as u16;
    cpu.push_word(ctx, psw);
    11
}

pub fn ori(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_byte(ctx);
    cpu.ora(value);
    7
}

pub fn rst_6(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.rst(ctx, 0x0030);
    11
}

pub fn rm(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    if cpu.return_if(ctx, !cpu.state.flags.sign) { 11 } else { 5 }
}

pub fn sphl(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.sp = cpu.state.hl();
    5
}

pub fn jm(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    cpu.jump_if(addr, !cpu.state.flags.sign);
    10
}

pub fn ei(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.enable_interrupts = true;
    4
}

pub fn cm(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let addr = cpu.fetch_word(ctx);
    if cpu.call_if(ctx, addr, !cpu.state.flags.sign) { 17 } else { 11 }
}

pub fn cpi(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    let value = cpu.fetch_byte(ctx);
    cpu.cmp(value);
    7
}

pub fn rst_7(cpu: &mut Cpu, ctx: &mut CpuContext) -> u8 {
    cpu.rst(ctx, 0x0038);
    11
}
