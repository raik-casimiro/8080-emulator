use crate::cpu::Cpu;

pub fn nop(cpu: &mut Cpu) {
    
}

pub fn lxi_b(cpu: &mut Cpu) {
    let value = cpu.fetch_word();

    cpu.state.set_bc(value);
}

pub fn stax_b(cpu: &mut Cpu) {
    let addr: u16 = cpu.state.bc();

    cpu.memory.write8(addr, cpu.state.a);
}

pub fn inx_b(cpu: &mut Cpu) {
    let bc: u16 = cpu.state.bc().wrapping_add(1);
    cpu.state.set_bc(bc);
}

pub fn inr_b(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.b, 1);

    cpu.state.b = cpu.state.b.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.b);
}

pub fn dcr_b(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.b, 1);

    cpu.state.b = cpu.state.b.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.b);
}

pub fn mvi_b(cpu: &mut Cpu) {
    cpu.state.b = cpu.fetch_byte();
}

pub fn rlc(cpu: &mut Cpu) {
    cpu.state.flags.carry = (cpu.state.a >> 7) == 1;
    cpu.state.a = cpu.state.a << 1;
    cpu.state.a = cpu.state.a | (cpu.state.flags.carry as u8);
}

pub fn dad_b(cpu: &mut Cpu) {
    let bc: u16 = cpu.state.bc();
    let hl: u16 = cpu.state.hl();
    let (result, overflow) = hl.overflowing_add(bc);

    cpu.state.flags.carry = overflow;

    cpu.state.set_hl(result);
}

pub fn ldax_b(cpu: &mut Cpu) {
    let addr: u16 = cpu.state.bc();
    cpu.state.a = cpu.memory.read8(addr);
}

pub fn dcx_b(cpu: &mut Cpu) {
    let bc: u16 = cpu.state.bc().wrapping_sub(1);
    cpu.state.set_bc(bc);
}

pub fn inr_c(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.c, 1);

    cpu.state.c = cpu.state.c.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.c);
}

pub fn dcr_c(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.c, 1);

    cpu.state.c = cpu.state.c.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.c);
}

pub fn mvi_c(cpu: &mut Cpu) {
    cpu.state.c = cpu.fetch_byte();
}

pub fn rrc(cpu: &mut Cpu) {
    let bit = cpu.state.a & 1;

    cpu.state.flags.carry = bit != 0;

    cpu.state.a >>= 1;
    cpu.state.a |= bit << 7;
}

pub fn lxi_d(cpu: &mut Cpu) {
    let value = cpu.fetch_word();
    cpu.state.set_de(value);
}

pub fn stax_d(cpu: &mut Cpu) {
    let addr: u16 = cpu.state.de();
    cpu.memory.write8(addr, cpu.state.a);
}

pub fn inx_d(cpu: &mut Cpu) {
    let de: u16 = cpu.state.de().wrapping_add(1);
    cpu.state.set_de(de);
}

pub fn inr_d(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.d, 1);

    cpu.state.d = cpu.state.d.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.d);
}

pub fn dcr_d(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.d, 1);

    cpu.state.d = cpu.state.d.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.d);
}

pub fn mvi_d(cpu: &mut Cpu) {
    cpu.state.d = cpu.fetch_byte();
}

pub fn ral(cpu: &mut Cpu) {
    let old_carry = cpu.state.flags.carry;

    cpu.state.flags.carry = (cpu.state.a >> 7) == 1;
    cpu.state.a = cpu.state.a << 1;
    cpu.state.a = cpu.state.a | (old_carry as u8);
}

pub fn dad_d(cpu: &mut Cpu) {
    let de: u16 = cpu.state.de();
    let hl: u16 = cpu.state.hl();
    let (result, overflow) = hl.overflowing_add(de);

    cpu.state.flags.carry = overflow;
    cpu.state.set_hl(result);
}

pub fn ldax_d(cpu: &mut Cpu) {
    let addr: u16 = cpu.state.de();
    cpu.state.a = cpu.memory.read8(addr);
}

pub fn dcx_d(cpu: &mut Cpu) {
    let de: u16 = cpu.state.de().wrapping_sub(1);
    cpu.state.set_de(de);
}

pub fn inr_e(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.e, 1);

    cpu.state.e = cpu.state.e.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.e);
}

pub fn dcr_e(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.e, 1);

    cpu.state.e = cpu.state.e.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.e);
}

pub fn mvi_e(cpu: &mut Cpu) {
    cpu.state.e = cpu.fetch_byte();
}

pub fn rar(cpu: &mut Cpu) {
    let old_carry = cpu.state.flags.carry;

    cpu.state.flags.carry = (cpu.state.a & 1) != 0;

    cpu.state.a >>= 1;

    if old_carry {
        cpu.state.a |= 0x80;
    }
}

pub fn lxi_h(cpu: &mut Cpu) {
    let value = cpu.fetch_word();
    cpu.state.set_hl(value);
}

pub fn shld(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    cpu.memory.write8(addr, cpu.state.l);
    cpu.memory.write8(addr + 1, cpu.state.h);
}

pub fn inx_h(cpu: &mut Cpu) {
    let hl: u16 = cpu.state.hl().wrapping_add(1);
    cpu.state.set_hl(hl);
}

pub fn inr_h(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.h, 1);

    cpu.state.h = cpu.state.h.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.h);
}

pub fn dcr_h(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.h, 1);

    cpu.state.h = cpu.state.h.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.h);
}

pub fn mvi_h(cpu: &mut Cpu) {
    cpu.state.h = cpu.fetch_byte();
}

pub fn daa(cpu: &mut Cpu) {
    println!("DAA not implemented");
}

pub fn dad_h(cpu: &mut Cpu) {
    let hl: u16 = cpu.state.hl();
    let (result, overflow) = hl.overflowing_add(hl);

    cpu.state.flags.carry = overflow;

    cpu.state.set_hl(result);
}

pub fn lhld(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    let lo = cpu.memory.read8(addr);
    let hi = cpu.memory.read8(addr + 1);

    cpu.state.l = lo;
    cpu.state.h = hi;
}

pub fn dcx_h(cpu: &mut Cpu) {
    let hl: u16 = cpu.state.hl().wrapping_sub(1);
    cpu.state.set_hl(hl);
}

pub fn inr_l(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.l, 1);

    cpu.state.l = cpu.state.l.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.l);
}

pub fn dcr_l(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.l, 1);

    cpu.state.l = cpu.state.l.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.l);
}

pub fn mvi_l(cpu: &mut Cpu) {
    cpu.state.l = cpu.fetch_byte();
}

pub fn cma(cpu: &mut Cpu) {
    cpu.state.a = !cpu.state.a;
}

pub fn lxi_sp(cpu: &mut Cpu) {
    let data = cpu.fetch_word();
    cpu.sp = data;
}

pub fn sta(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();
    cpu.memory.write8(addr, cpu.state.a);
}

pub fn inx_sp(cpu: &mut Cpu) {
    cpu.sp = cpu.sp.wrapping_add(1);
}

pub fn inr_m(cpu: &mut Cpu) {
    let value = cpu.read_m();

    cpu.state.flags.update_auxiliary_carry_add(value, 1);

    let new_value = value.wrapping_add(1);

    cpu.write_m(new_value);
    cpu.state.flags.update_szp(new_value);
}

pub fn dcr_m(cpu: &mut Cpu) {
    let value = cpu.read_m();

    cpu.state.flags.update_auxiliary_carry_sub(value, 1);

    let new_value = value.wrapping_sub(1);

    cpu.write_m(new_value);
    cpu.state.flags.update_szp(new_value);
}

pub fn mvi_m(cpu: &mut Cpu) {
    let value = cpu.fetch_byte();
    cpu.write_m(value);
}

pub fn stc(cpu: &mut Cpu) {
    cpu.state.flags.carry = true;
}

pub fn dad_sp(cpu: &mut Cpu) {
    let hl: u16 = cpu.state.hl();
    let (result, overflow) = hl.overflowing_add(cpu.sp);

    cpu.state.flags.carry = overflow;

    cpu.state.set_hl(result);
}

pub fn lda(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();
    cpu.state.a = cpu.memory.read8(addr);
}

pub fn dcx_sp(cpu: &mut Cpu) {
    cpu.sp = cpu.sp.wrapping_sub(1);
}

pub fn inr_a(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_add(cpu.state.a, 1);

    cpu.state.a = cpu.state.a.wrapping_add(1);
    cpu.state.flags.update_szp(cpu.state.a);
}

pub fn dcr_a(cpu: &mut Cpu) {
    cpu.state.flags.update_auxiliary_carry_sub(cpu.state.a, 1);

    cpu.state.a = cpu.state.a.wrapping_sub(1);
    cpu.state.flags.update_szp(cpu.state.a);
}

pub fn mvi_a(cpu: &mut Cpu) {
    cpu.state.a = cpu.fetch_byte();
}

pub fn cmc(cpu: &mut Cpu) {
    cpu.state.flags.carry = !cpu.state.flags.carry;
}

pub fn mov_bb(cpu: &mut Cpu) {
    cpu.state.b = cpu.state.b;
}

pub fn mov_bc(cpu: &mut Cpu) {
    cpu.state.b = cpu.state.c;
}

pub fn mov_bd(cpu: &mut Cpu) {
    cpu.state.b = cpu.state.d;
}

pub fn mov_be(cpu: &mut Cpu) {
    cpu.state.b = cpu.state.e;
}

pub fn mov_bh(cpu: &mut Cpu) {
    cpu.state.b = cpu.state.h;
}

pub fn mov_bl(cpu: &mut Cpu) {
    cpu.state.b = cpu.state.l;
}

pub fn mov_bm(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.b = value;
}

pub fn mov_ba(cpu: &mut Cpu) {
    cpu.state.b = cpu.state.a;
}

pub fn mov_cb(cpu: &mut Cpu) {
    cpu.state.c = cpu.state.b;
}

pub fn mov_cc(cpu: &mut Cpu) {
    cpu.state.c = cpu.state.c;
}

pub fn mov_cd(cpu: &mut Cpu) {
    cpu.state.c = cpu.state.d;
}

pub fn mov_ce(cpu: &mut Cpu) {
    cpu.state.c = cpu.state.e;
}

pub fn mov_ch(cpu: &mut Cpu) {
    cpu.state.c = cpu.state.h;
}

pub fn mov_cl(cpu: &mut Cpu) {
    cpu.state.c = cpu.state.l;
}

pub fn mov_cm(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.c = value;
}

pub fn mov_ca(cpu: &mut Cpu) {
    cpu.state.c = cpu.state.a;
}

pub fn mov_db(cpu: &mut Cpu) {
    cpu.state.d = cpu.state.b;
}

pub fn mov_dc(cpu: &mut Cpu) {
    cpu.state.d = cpu.state.c;
}

pub fn mov_dd(cpu: &mut Cpu) {
    cpu.state.d = cpu.state.d;
}

pub fn mov_de(cpu: &mut Cpu) {
    cpu.state.d = cpu.state.e;
}

pub fn mov_dh(cpu: &mut Cpu) {
    cpu.state.d = cpu.state.h;
}

pub fn mov_dl(cpu: &mut Cpu) {
    cpu.state.d = cpu.state.l;
}

pub fn mov_dm(cpu: &mut Cpu) {
    let value= cpu.read_m();
    cpu.state.d = value;
}

pub fn mov_da(cpu: &mut Cpu) {
    cpu.state.d = cpu.state.a;
}

pub fn mov_eb(cpu: &mut Cpu) {
    cpu.state.e = cpu.state.b;
}

pub fn mov_ec(cpu: &mut Cpu) {
    cpu.state.e = cpu.state.c;
}

pub fn mov_ed(cpu: &mut Cpu) {
    cpu.state.e = cpu.state.d;
}

pub fn mov_ee(cpu: &mut Cpu) {
    cpu.state.e = cpu.state.e;
}

pub fn mov_eh(cpu: &mut Cpu) {
    cpu.state.e = cpu.state.h;
}

pub fn mov_el(cpu: &mut Cpu) {
    cpu.state.e = cpu.state.l;
}

pub fn mov_em(cpu: &mut Cpu) {
    let value= cpu.read_m();
    cpu.state.e = value;
}

pub fn mov_ea(cpu: &mut Cpu) {
    cpu.state.e = cpu.state.a;
}

pub fn mov_hb(cpu: &mut Cpu) {
    cpu.state.h = cpu.state.b;
}

pub fn mov_hc(cpu: &mut Cpu) {
    cpu.state.h = cpu.state.c;
}

pub fn mov_hd(cpu: &mut Cpu) {
    cpu.state.h = cpu.state.d;
}

pub fn mov_he(cpu: &mut Cpu) {
    cpu.state.h = cpu.state.e;
}

pub fn mov_hh(cpu: &mut Cpu) {
    cpu.state.h = cpu.state.h;
}

pub fn mov_hl(cpu: &mut Cpu) {
    cpu.state.h = cpu.state.l;
}

pub fn mov_hm(cpu: &mut Cpu) {
    let value= cpu.read_m();
    cpu.state.h = value;
}

pub fn mov_ha(cpu: &mut Cpu) {
    cpu.state.h = cpu.state.a;
}

pub fn mov_lb(cpu: &mut Cpu) {
    cpu.state.l = cpu.state.b;
}

pub fn mov_lc(cpu: &mut Cpu) {
    cpu.state.l = cpu.state.c;
}

pub fn mov_ld(cpu: &mut Cpu) {
    cpu.state.l = cpu.state.d;
}

pub fn mov_le(cpu: &mut Cpu) {
    cpu.state.l = cpu.state.e;
}

pub fn mov_lh(cpu: &mut Cpu) {
    cpu.state.l = cpu.state.h;
}

pub fn mov_ll(cpu: &mut Cpu) {
    cpu.state.l = cpu.state.l;
}

pub fn mov_lm(cpu: &mut Cpu) {
    let value= cpu.read_m();
    cpu.state.l = value;
}

pub fn mov_la(cpu: &mut Cpu) {
    cpu.state.l = cpu.state.a;
}

pub fn mov_mb(cpu: &mut Cpu) {
    cpu.write_m(cpu.state.b);
}

pub fn mov_mc(cpu: &mut Cpu) {
    cpu.write_m(cpu.state.c);
}

pub fn mov_md(cpu: &mut Cpu) {
    cpu.write_m(cpu.state.d);
}

pub fn mov_me(cpu: &mut Cpu) {
    cpu.write_m(cpu.state.e);
}

pub fn mov_mh(cpu: &mut Cpu) {
    cpu.write_m(cpu.state.h);
}

pub fn mov_ml(cpu: &mut Cpu) {
    cpu.write_m(cpu.state.l);
}

pub fn hlt(cpu: &mut Cpu) {
    cpu.halted = true;
}

pub fn mov_ma(cpu: &mut Cpu) {
    cpu.write_m(cpu.state.a);
}

pub fn mov_ab(cpu: &mut Cpu) {
    cpu.state.a = cpu.state.b;
}

pub fn mov_ac(cpu: &mut Cpu) {
    cpu.state.a = cpu.state.c;
}

pub fn mov_ad(cpu: &mut Cpu) {
    cpu.state.a = cpu.state.d;
}

pub fn mov_ae(cpu: &mut Cpu) {
    cpu.state.a = cpu.state.e;
}

pub fn mov_ah(cpu: &mut Cpu) {
    cpu.state.a = cpu.state.h;
}

pub fn mov_al(cpu: &mut Cpu) {
    cpu.state.a = cpu.state.l;
}

pub fn mov_am(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.a = value;
}

pub fn mov_aa(cpu: &mut Cpu) {
    cpu.state.a = cpu.state.a;
}

pub fn add_b(cpu: &mut Cpu) {
    cpu.state.add(cpu.state.b);
}

pub fn add_c(cpu: &mut Cpu) {
    cpu.state.add(cpu.state.c);
}

pub fn add_d(cpu: &mut Cpu) {
    cpu.state.add(cpu.state.d);
}

pub fn add_e(cpu: &mut Cpu) {
    cpu.state.add(cpu.state.e);
}

pub fn add_h(cpu: &mut Cpu) {
    cpu.state.add(cpu.state.h);
}

pub fn add_l(cpu: &mut Cpu) {
    cpu.state.add(cpu.state.l);
}

pub fn add_m(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.add(value);
}

pub fn add_a(cpu: &mut Cpu) {
    cpu.state.add(cpu.state.a);
}

pub fn adc_b(cpu: &mut Cpu) {
    cpu.state.adc(cpu.state.b);
}

pub fn adc_c(cpu: &mut Cpu) {
    cpu.state.adc(cpu.state.c);
}

pub fn adc_d(cpu: &mut Cpu) {
    cpu.state.adc(cpu.state.d);
}

pub fn adc_e(cpu: &mut Cpu) {
    cpu.state.adc(cpu.state.e);
}

pub fn adc_h(cpu: &mut Cpu) {
    cpu.state.adc(cpu.state.h);
}

pub fn adc_l(cpu: &mut Cpu) {
    cpu.state.adc(cpu.state.l);
}

pub fn adc_m(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.adc(value);
}

pub fn adc_a(cpu: &mut Cpu) {
    cpu.state.adc(cpu.state.a);
}

pub fn sub_b(cpu: &mut Cpu) {
    cpu.state.sub(cpu.state.b);
}

pub fn sub_c(cpu: &mut Cpu) {
    cpu.state.sub(cpu.state.c);
}

pub fn sub_d(cpu: &mut Cpu) {
    cpu.state.sub(cpu.state.d);
}

pub fn sub_e(cpu: &mut Cpu) {
    cpu.state.sub(cpu.state.e);
}

pub fn sub_h(cpu: &mut Cpu) {
    cpu.state.sub(cpu.state.h);
}

pub fn sub_l(cpu: &mut Cpu) {
    cpu.state.sub(cpu.state.l);
}

pub fn sub_m(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.sub(value);
}

pub fn sub_a(cpu: &mut Cpu) {
    cpu.state.sub(cpu.state.a);
}

pub fn sbb_b(cpu: &mut Cpu) {
    cpu.state.sbb(cpu.state.b);
}

pub fn sbb_c(cpu: &mut Cpu) {
    cpu.state.sbb(cpu.state.c);
}

pub fn sbb_d(cpu: &mut Cpu) {
    cpu.state.sbb(cpu.state.d);
}

pub fn sbb_e(cpu: &mut Cpu) {
    cpu.state.sbb(cpu.state.e);
}

pub fn sbb_h(cpu: &mut Cpu) {
    cpu.state.sbb(cpu.state.h);
}

pub fn sbb_l(cpu: &mut Cpu) {
    cpu.state.sbb(cpu.state.l);
}

pub fn sbb_m(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.sbb(value);
}

pub fn sbb_a(cpu: &mut Cpu) {
    cpu.state.sbb(cpu.state.a);
}

pub fn ana_b(cpu: &mut Cpu) {
    cpu.state.ana(cpu.state.b);
}

pub fn ana_c(cpu: &mut Cpu) {
    cpu.state.ana(cpu.state.c);
}

pub fn ana_d(cpu: &mut Cpu) {
    cpu.state.ana(cpu.state.d);
}

pub fn ana_e(cpu: &mut Cpu) {
    cpu.state.ana(cpu.state.e);
}

pub fn ana_h(cpu: &mut Cpu) {
    cpu.state.ana(cpu.state.h);
}

pub fn ana_l(cpu: &mut Cpu) {
    cpu.state.ana(cpu.state.l);
}

pub fn ana_m(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.ana(value);
}

pub fn ana_a(cpu: &mut Cpu) {
    cpu.state.ana(cpu.state.a);
}

pub fn xra_b(cpu: &mut Cpu) {
    cpu.state.xra(cpu.state.b);
}

pub fn xra_c(cpu: &mut Cpu) {
    cpu.state.xra(cpu.state.c);
}

pub fn xra_d(cpu: &mut Cpu) {
    cpu.state.xra(cpu.state.d);
}

pub fn xra_e(cpu: &mut Cpu) {
    cpu.state.xra(cpu.state.e);
}

pub fn xra_h(cpu: &mut Cpu) {
    cpu.state.xra(cpu.state.h);
}

pub fn xra_m(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.xra(value);
}

pub fn xra_l(cpu: &mut Cpu) {
    cpu.state.xra(cpu.state.l);
}

pub fn xra_a(cpu: &mut Cpu) {
    cpu.state.xra(cpu.state.a);
}

pub fn ora_b(cpu: &mut Cpu) {
    cpu.state.ora(cpu.state.b);
}

pub fn ora_c(cpu: &mut Cpu) {
    cpu.state.ora(cpu.state.c);
}

pub fn ora_d(cpu: &mut Cpu) {
    cpu.state.ora(cpu.state.d);
}

pub fn ora_e(cpu: &mut Cpu) {
    cpu.state.ora(cpu.state.e);
}

pub fn ora_h(cpu: &mut Cpu) {
    cpu.state.ora(cpu.state.h);
}

pub fn ora_l(cpu: &mut Cpu) {
    cpu.state.ora(cpu.state.l);
}

pub fn ora_m(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.ora(value);
}

pub fn ora_a(cpu: &mut Cpu) {
    cpu.state.ora(cpu.state.a);
}

pub fn cmp_b(cpu: &mut Cpu) {
    cpu.state.cmp(cpu.state.b);
}

pub fn cmp_c(cpu: &mut Cpu) {
    cpu.state.cmp(cpu.state.c);
}

pub fn cmp_d(cpu: &mut Cpu) {
    cpu.state.cmp(cpu.state.d);
}

pub fn cmp_e(cpu: &mut Cpu) {
    cpu.state.cmp(cpu.state.e);
}

pub fn cmp_h(cpu: &mut Cpu) {
    cpu.state.cmp(cpu.state.h);
}

pub fn cmp_l(cpu: &mut Cpu) {
    cpu.state.cmp(cpu.state.l);
}

pub fn cmp_m(cpu: &mut Cpu) {
    let value = cpu.read_m();
    cpu.state.cmp(value);
}

pub fn cmp_a(cpu: &mut Cpu) {
    cpu.state.cmp(cpu.state.a);
}

pub fn rnz(cpu: &mut Cpu) {
    if cpu.state.flags.zero {
        return;
    }

    cpu.pc = cpu.pop_word();
}

pub fn pop_bc(cpu: &mut Cpu) {
    let value = cpu.pop_word();
    cpu.state.set_bc(value);
}

pub fn jnz(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if (cpu.state.flags.zero) {
        return;
    }

    cpu.pc = addr;
}

pub fn jmp(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();
    cpu.pc = addr;
}

pub fn cnz(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if (cpu.state.flags.zero) {
        return;
    }

    cpu.push_word(cpu.pc);
    cpu.pc = addr;
}

pub fn push_bc(cpu: &mut Cpu) {
    cpu.push_word( cpu.state.bc())
}

pub fn adi(cpu: &mut Cpu) {
    let value = cpu.fetch_byte();
    cpu.state.add(value);
}

pub fn rst_0(cpu: &mut Cpu) {
    cpu.rst(0x0000)
}

pub fn rz(cpu: &mut Cpu) {
    if !cpu.state.flags.zero {
        return;
    }

    cpu.pc = cpu.pop_word();
}

pub fn ret(cpu: &mut Cpu) {
    cpu.pc = cpu.pop_word();
}

pub fn jz(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if !cpu.state.flags.zero {
        return;
    }

    cpu.pc = addr;
}

pub fn cz(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if !cpu.state.flags.zero {
        return;
    }

    cpu.push_word(cpu.pc);
    cpu.pc = addr;
}

pub fn call(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();
    cpu.push_word(cpu.pc);
    cpu.pc = addr;
}

pub fn aci(cpu: &mut Cpu) {
    let value = cpu.fetch_byte();
    cpu.state.adc(value);
}

pub fn rst_1(cpu: &mut Cpu) {
    cpu.rst(0x0008)
}

pub fn rnc(cpu: &mut Cpu) {
    if cpu.state.flags.carry {
        return;
    }

    cpu.pc = cpu.pop_word();
}

pub fn pop_de(cpu: &mut Cpu) {
    let value = cpu.pop_word();
    cpu.state.set_de(value);
}

pub fn jnc(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if cpu.state.flags.carry {
        return;
    }

    cpu.pc = addr;
}

pub fn out(cpu: &mut Cpu) {
    let port = cpu.fetch_byte();
    cpu.io.write(port, cpu.state.a);
}

pub fn cnc(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if cpu.state.flags.carry {
        return;
    }

    cpu.push_word(cpu.pc);
    cpu.pc = addr;
}

pub fn push_de(cpu: &mut Cpu) {
    cpu.push_word(cpu.state.de());
}

pub fn sui(cpu: &mut Cpu) {
    let value = cpu.fetch_byte();
    cpu.state.sub(value);
}

pub fn rst_2(cpu: &mut Cpu) {
    cpu.rst(0x0010)
}

pub fn rc(cpu: &mut Cpu) {
    if !cpu.state.flags.carry {
        return;
    }

    cpu.pc = cpu.pop_word();
}

pub fn jc(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if !cpu.state.flags.carry {
        return;
    }

    cpu.pc = addr;
}

pub fn in_(cpu: &mut Cpu) {
    let port = cpu.fetch_byte();
    cpu.state.a = cpu.io.read(port);
}

pub fn cc(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if !cpu.state.flags.carry {
        return;
    }

    cpu.push_word(cpu.pc);
    cpu.pc = addr;
}

pub fn sbi(cpu: &mut Cpu) {
    let value = cpu.fetch_byte();
    cpu.state.sbb(value);
}

pub fn rst_3(cpu: &mut Cpu) {
    cpu.rst(0x0018)
}

pub fn rpo(cpu: &mut Cpu) {
    if cpu.state.flags.parity {
        return;
    }

    cpu.pc = cpu.pop_word();
}

pub fn pop_hl(cpu: &mut Cpu) {
    let value = cpu.pop_word();
    cpu.state.set_hl(value);
}

pub fn jpo(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if cpu.state.flags.parity {
        return;
    }

    cpu.pc = addr;
}

pub fn xthl(cpu: &mut Cpu) {
    let stack_value = cpu.pop_word();
    let hl = cpu.state.hl();

    cpu.push_word(hl);
    cpu.state.set_hl(stack_value);
}

pub fn cpo(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if cpu.state.flags.parity {
        return;
    }

    cpu.push_word(cpu.pc);
    cpu.pc = addr;
}

pub fn push_hl(cpu: &mut Cpu) {
    cpu.push_word(cpu.state.hl());
}

pub fn ani(cpu: &mut Cpu) {
    let value = cpu.fetch_byte();
    cpu.state.ana(value);
}

pub fn rst_4(cpu: &mut Cpu) {
    cpu.rst(0x0020)
}

pub fn rpe(cpu: &mut Cpu) {
    if !cpu.state.flags.parity {
        return;
    }

    cpu.pc = cpu.pop_word();
}

pub fn pchl(cpu: &mut Cpu) {
    let hl = cpu.state.hl();
    cpu.pc = hl;
}

pub fn jpe(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if !cpu.state.flags.parity {
        return;
    }

    cpu.pc = addr;
}

pub fn xchg(cpu: &mut Cpu) {
    let hl = cpu.state.hl();
    let de = cpu.state.de();

    cpu.state.set_hl(de);
    cpu.state.set_de(hl);
}

pub fn cpe(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if !cpu.state.flags.parity {
        return;
    }

    cpu.push_word(cpu.pc);
    cpu.pc = addr;
}

pub fn xri(cpu: &mut Cpu) {
    let value = cpu.fetch_byte();
    cpu.state.xra(value);
}

pub fn rst_5(cpu: &mut Cpu) {
    cpu.rst(0x0028)
}

pub fn rp(cpu: &mut Cpu) {
    if cpu.state.flags.sign {
        return;
    }

    cpu.pc = cpu.pop_word();
}

pub fn pop_psw(cpu: &mut Cpu) {
    let psw = cpu.pop_word();

    cpu.state.flags.unpack(psw as u8);
    cpu.state.a = (psw >> 8) as u8;
}

pub fn jp(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if cpu.state.flags.sign {
        return;
    }

    cpu.pc = addr;
}

pub fn di(cpu: &mut Cpu) {
    cpu.enable_interrupts = false;
}

pub fn cp(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if cpu.state.flags.sign {
        return;
    }

    cpu.push_word(cpu.pc);
    cpu.pc = addr;
}

pub fn push_psw(cpu: &mut Cpu) {
    let psw = ((cpu.state.a as u16) << 8) | cpu.state.flags.pack() as u16;
    cpu.push_word(psw);
}

pub fn ori(cpu: &mut Cpu) {
    let value = cpu.fetch_byte();
    cpu.state.ora(value);
}

pub fn rst_6(cpu: &mut Cpu) {
    cpu.rst(0x0030)
}

pub fn rm(cpu: &mut Cpu) {
    if !cpu.state.flags.sign {
        return;
    }

    cpu.pc = cpu.pop_word();
}

pub fn sphl(cpu: &mut Cpu) {
    cpu.sp = cpu.state.hl();
}

pub fn jm(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if !cpu.state.flags.sign {
        return;
    }

    cpu.pc = addr;
}

pub fn ei(cpu: &mut Cpu) {
    cpu.enable_interrupts = true;
}

pub fn cm(cpu: &mut Cpu) {
    let addr = cpu.fetch_word();

    if !cpu.state.flags.sign {
        return;
    }

    cpu.push_word(cpu.pc);
    cpu.pc = addr;
}

pub fn cpi(cpu: &mut Cpu) {
    let value = cpu.fetch_byte();
    cpu.state.cmp(value);
}

pub fn rst_7(cpu: &mut Cpu) {
    cpu.rst(0x0038)
}