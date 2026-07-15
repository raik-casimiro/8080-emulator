use crate::cpu::{Cpu, CpuContext};
use crate::instructions::*;

pub struct Instruction {
    pub mnemonic: &'static str,
    pub cycles: u8,
    pub execute: fn(&mut Cpu, ctx: &mut CpuContext) -> u8,
}

pub static OPCODES: [Instruction; 256] = [
    Instruction {
        mnemonic: "NOP",
        cycles: 4,
        execute: nop,
    },
    Instruction {
        mnemonic: "LXI B",
        cycles: 10,
        execute: lxi_b,
    },
    Instruction {
        mnemonic: "STAX B",
        cycles: 7,
        execute: stax_b,
    },
    Instruction {
        mnemonic: "INX B",
        cycles: 5,
        execute: inx_b,
    },
    Instruction {
        mnemonic: "INR B",
        cycles: 5,
        execute: inr_b,
    },
    Instruction {
        mnemonic: "DCR B",
        cycles: 5,
        execute: dcr_b,
    },
    Instruction {
        mnemonic: "MVI B",
        cycles: 7,
        execute: mvi_b,
    },
    Instruction {
        mnemonic: "RLC",
        cycles: 7,
        execute: rlc,
    },
    Instruction {
        mnemonic: "NOP",
        cycles: 4,
        execute: nop,
    },
    Instruction {
        mnemonic: "DAD_B",
        cycles: 10,
        execute: dad_b,
    },
    Instruction {
        mnemonic: "LDAX_B",
        cycles: 7,
        execute: ldax_b,
    },
    Instruction {
        mnemonic: "DCX_B",
        cycles: 5,
        execute: dcx_b,
    },
    Instruction {
        mnemonic: "INR_C",
        cycles: 5,
        execute: inr_c,
    },
    Instruction {
        mnemonic: "DCR_C",
        cycles: 5,
        execute: dcr_c,
    },
    Instruction {
        mnemonic: "MVI_C",
        cycles: 7,
        execute: mvi_c,
    },
    Instruction {
        mnemonic: "RRC",
        cycles: 4,
        execute: rrc,
    },
    Instruction {
        mnemonic: "NOP",
        cycles: 4,
        execute: nop,
    },
    Instruction {
        mnemonic: "LXI_D",
        cycles: 10,
        execute: lxi_d,
    },
    Instruction {
        mnemonic: "STAX_D",
        cycles: 7,
        execute: stax_d,
    },
    Instruction {
        mnemonic: "INX_D",
        cycles: 5,
        execute: inx_d,
    },
    Instruction {
        mnemonic: "INR_D",
        cycles: 5,
        execute: inr_d,
    },
    Instruction {
        mnemonic: "DCR_D",
        cycles: 5,
        execute: dcr_d,
    },
    Instruction {
        mnemonic: "MVI_D",
        cycles: 7,
        execute: mvi_d,
    },
    Instruction {
        mnemonic: "RAL",
        cycles: 4,
        execute: ral,
    },
    Instruction {
        mnemonic: "NOP",
        cycles: 4,
        execute: nop,
    },
    Instruction {
        mnemonic: "DAD_D",
        cycles: 10,
        execute: dad_d,
    },
    Instruction {
        mnemonic: "LDAX_D",
        cycles: 7,
        execute: ldax_d,
    },
    Instruction {
        mnemonic: "DCX_D",
        cycles: 5,
        execute: dcx_d,
    },
    Instruction {
        mnemonic: "INR_E",
        cycles: 5,
        execute: inr_e,
    },
    Instruction {
        mnemonic: "DCR_E",
        cycles: 5,
        execute: dcr_e,
    },
    Instruction {
        mnemonic: "MVI_E",
        cycles: 7,
        execute: mvi_e,
    },
    Instruction {
        mnemonic: "RAR",
        cycles: 4,
        execute: rar,
    },
    Instruction {
        mnemonic: "NOP",
        cycles: 4,
        execute: nop,
    },
    Instruction {
        mnemonic: "LXI_H",
        cycles: 10,
        execute: lxi_h,
    },
    Instruction {
        mnemonic: "SHLD",
        cycles: 16,
        execute: shld,
    },
    Instruction {
        mnemonic: "INX_H",
        cycles: 5,
        execute: inx_h,
    },
    Instruction {
        mnemonic: "INR_H",
        cycles: 5,
        execute: inr_h,
    },
    Instruction {
        mnemonic: "DCR_H",
        cycles: 5,
        execute: dcr_h,
    },
    Instruction {
        mnemonic: "MVI_H",
        cycles: 7,
        execute: mvi_h,
    },
    Instruction {
        mnemonic: "DAA",
        cycles: 4,
        execute: daa,
    },
    Instruction {
        mnemonic: "NOP",
        cycles: 4,
        execute: nop,
    },
    Instruction {
        mnemonic: "DAD_H",
        cycles: 10,
        execute: dad_h,
    },
    Instruction {
        mnemonic: "LHLD",
        cycles: 16,
        execute: lhld,
    },
    Instruction {
        mnemonic: "DCX_H",
        cycles: 5,
        execute: dcx_h,
    },
    Instruction {
        mnemonic: "INR_L",
        cycles: 5,
        execute: inr_l,
    },
    Instruction {
        mnemonic: "DCR_L",
        cycles: 5,
        execute: dcr_l,
    },
    Instruction {
        mnemonic: "MVI_L",
        cycles: 7,
        execute: mvi_l,
    },
    Instruction {
        mnemonic: "CMA",
        cycles: 4,
        execute: cma,
    },
    Instruction {
        mnemonic: "NOP",
        cycles: 4,
        execute: nop,
    },
    Instruction {
        mnemonic: "LXI_SP",
        cycles: 10,
        execute: lxi_sp,
    },
    Instruction {
        mnemonic: "STA",
        cycles: 13,
        execute: sta,
    },
    Instruction {
        mnemonic: "INX_SP",
        cycles: 5,
        execute: inx_sp,
    },
    Instruction {
        mnemonic: "INR_M",
        cycles: 10,
        execute: inr_m,
    },
    Instruction {
        mnemonic: "DCR_M",
        cycles: 10,
        execute: dcr_m,
    },
    Instruction {
        mnemonic: "MVI_M",
        cycles: 10,
        execute: mvi_m,
    },
    Instruction {
        mnemonic: "STC",
        cycles: 4,
        execute: stc,
    },
    Instruction {
        mnemonic: "NOP",
        cycles: 4,
        execute: nop,
    },
    Instruction {
        mnemonic: "DAD_SP",
        cycles: 10,
        execute: dad_sp,
    },
    Instruction {
        mnemonic: "LDA",
        cycles: 13,
        execute: lda,
    },
    Instruction {
        mnemonic: "DCX_SP",
        cycles: 5,
        execute: dcx_sp,
    },
    Instruction {
        mnemonic: "INR_A",
        cycles: 5,
        execute: inr_a,
    },
    Instruction {
        mnemonic: "DCR_A",
        cycles: 5,
        execute: dcr_a,
    },
    Instruction {
        mnemonic: "MVI_A",
        cycles: 7,
        execute: mvi_a,
    },
    Instruction {
        mnemonic: "CMC",
        cycles: 4,
        execute: cmc,
    },
    Instruction {
        mnemonic: "MOV_B_B",
        cycles: 5,
        execute: mov_bb,
    },
    Instruction {
        mnemonic: "MOV_B_C",
        cycles: 5,
        execute: mov_bc,
    },
    Instruction {
        mnemonic: "MOV_B_D",
        cycles: 5,
        execute: mov_bd,
    },
    Instruction {
        mnemonic: "MOV_B_E",
        cycles: 5,
        execute: mov_be,
    },
    Instruction {
        mnemonic: "MOV_B_H",
        cycles: 5,
        execute: mov_bh,
    },
    Instruction {
        mnemonic: "MOV_B_L",
        cycles: 5,
        execute: mov_bl,
    },
    Instruction {
        mnemonic: "MOV_B_M",
        cycles: 7,
        execute: mov_bm,
    },
    Instruction {
        mnemonic: "MOV_B_A",
        cycles: 5,
        execute: mov_ba,
    },
    Instruction {
        mnemonic: "MOV_C_B",
        cycles: 5,
        execute: mov_cb,
    },
    Instruction {
        mnemonic: "MOV_C_C",
        cycles: 5,
        execute: mov_cc,
    },
    Instruction {
        mnemonic: "MOV_C_D",
        cycles: 5,
        execute: mov_cd,
    },
    Instruction {
        mnemonic: "MOV_C_E",
        cycles: 5,
        execute: mov_ce,
    },
    Instruction {
        mnemonic: "MOV_C_H",
        cycles: 5,
        execute: mov_ch,
    },
    Instruction {
        mnemonic: "MOV_C_L",
        cycles: 5,
        execute: mov_cl,
    },
    Instruction {
        mnemonic: "MOV_C_M",
        cycles: 7,
        execute: mov_cm,
    },
    Instruction {
        mnemonic: "MOV_C_A",
        cycles: 5,
        execute: mov_ca,
    },
    Instruction {
        mnemonic: "MOV_D_B",
        cycles: 5,
        execute: mov_db,
    },
    Instruction {
        mnemonic: "MOV_D_C",
        cycles: 5,
        execute: mov_dc,
    },
    Instruction {
        mnemonic: "MOV_D_D",
        cycles: 5,
        execute: mov_dd,
    },
    Instruction {
        mnemonic: "MOV_D_E",
        cycles: 5,
        execute: mov_de,
    },
    Instruction {
        mnemonic: "MOV_D_H",
        cycles: 5,
        execute: mov_dh,
    },
    Instruction {
        mnemonic: "MOV_D_L",
        cycles: 5,
        execute: mov_dl,
    },
    Instruction {
        mnemonic: "MOV_D_M",
        cycles: 7,
        execute: mov_dm,
    },
    Instruction {
        mnemonic: "MOV_D_A",
        cycles: 5,
        execute: mov_da,
    },
    Instruction {
        mnemonic: "MOV_E_B",
        cycles: 5,
        execute: mov_eb,
    },
    Instruction {
        mnemonic: "MOV_E_C",
        cycles: 5,
        execute: mov_ec,
    },
    Instruction {
        mnemonic: "MOV_E_D",
        cycles: 5,
        execute: mov_ed,
    },
    Instruction {
        mnemonic: "MOV_E_E",
        cycles: 5,
        execute: mov_ee,
    },
    Instruction {
        mnemonic: "MOV_E_h",
        cycles: 5,
        execute: mov_eh,
    },
    Instruction {
        mnemonic: "MOV_E_L",
        cycles: 5,
        execute: mov_el,
    },
    Instruction {
        mnemonic: "MOV_E_M",
        cycles: 7,
        execute: mov_em,
    },
    Instruction {
        mnemonic: "MOV_E_A",
        cycles: 5,
        execute: mov_ea,
    },
    Instruction {
        mnemonic: "MOV_H_B",
        cycles: 5,
        execute: mov_hb,
    },
    Instruction {
        mnemonic: "MOV_H_C",
        cycles: 5,
        execute: mov_hc,
    },
    Instruction {
        mnemonic: "MOV_H_D",
        cycles: 5,
        execute: mov_hd,
    },
    Instruction {
        mnemonic: "MOV_H_E",
        cycles: 5,
        execute: mov_he,
    },
    Instruction {
        mnemonic: "MOV_H_H",
        cycles: 5,
        execute: mov_hh,
    },
    Instruction {
        mnemonic: "MOV_H_L",
        cycles: 5,
        execute: mov_hl,
    },
    Instruction {
        mnemonic: "MOV_H_M",
        cycles: 7,
        execute: mov_hm,
    },
    Instruction {
        mnemonic: "MOV_H_A",
        cycles: 5,
        execute: mov_ha,
    },
    Instruction {
        mnemonic: "MOV_L_B",
        cycles: 5,
        execute: mov_lb,
    },
    Instruction {
        mnemonic: "MOV_L_C",
        cycles: 5,
        execute: mov_lc,
    },
    Instruction {
        mnemonic: "MOV_L_D",
        cycles: 5,
        execute: mov_ld,
    },
    Instruction {
        mnemonic: "MOV_L_E",
        cycles: 5,
        execute: mov_le,
    },
    Instruction {
        mnemonic: "MOV_L_H",
        cycles: 5,
        execute: mov_lh,
    },
    Instruction {
        mnemonic: "MOV_L_L",
        cycles: 5,
        execute: mov_ll,
    },
    Instruction {
        mnemonic: "MOV_L_M",
        cycles: 7,
        execute: mov_lm,
    },
    Instruction {
        mnemonic: "MOV_L_A",
        cycles: 5,
        execute: mov_la,
    },
    Instruction {
        mnemonic: "MOV_M_B",
        cycles: 5,
        execute: mov_mb,
    },
    Instruction {
        mnemonic: "MOV_M_C",
        cycles: 5,
        execute: mov_mc,
    },
    Instruction {
        mnemonic: "MOV_M_D",
        cycles: 5,
        execute: mov_md,
    },
    Instruction {
        mnemonic: "MOV_M_E",
        cycles: 5,
        execute: mov_me,
    },
    Instruction {
        mnemonic: "MOV_M_H",
        cycles: 5,
        execute: mov_mh,
    },
    Instruction {
        mnemonic: "MOV_M_L",
        cycles: 5,
        execute: mov_ml,
    },
    Instruction {
        mnemonic: "HLT",
        cycles: 5,
        execute: hlt,
    },
    Instruction {
        mnemonic: "MOV_M_A",
        cycles: 5,
        execute: mov_ma,
    },
    Instruction {
        mnemonic: "MOV_A_B",
        cycles: 5,
        execute: mov_ab,
    },
    Instruction {
        mnemonic: "MOV_A_C",
        cycles: 5,
        execute: mov_ac,
    },
    Instruction {
        mnemonic: "MOV_A_D",
        cycles: 5,
        execute: mov_ad,
    },
    Instruction {
        mnemonic: "MOV_A_E",
        cycles: 5,
        execute: mov_ae,
    },
    Instruction {
        mnemonic: "MOV_A_h",
        cycles: 5,
        execute: mov_ah,
    },
    Instruction {
        mnemonic: "MOV_A_L",
        cycles: 5,
        execute: mov_al,
    },
    Instruction {
        mnemonic: "MOV_A_M",
        cycles: 7,
        execute: mov_am,
    },
    Instruction {
        mnemonic: "MOV_A_A",
        cycles: 5,
        execute: mov_aa,
    },
    Instruction {
        mnemonic: "ADD_B",
        cycles: 4,
        execute: add_b,
    },
    Instruction {
        mnemonic: "ADD_C",
        cycles: 4,
        execute: add_c,
    },
    Instruction {
        mnemonic: "ADD_D",
        cycles: 4,
        execute: add_d,
    },
    Instruction {
        mnemonic: "ADD_E",
        cycles: 4,
        execute: add_e,
    },
    Instruction {
        mnemonic: "ADD_H",
        cycles: 4,
        execute: add_h,
    },
    Instruction {
        mnemonic: "ADD_L",
        cycles: 4,
        execute: add_l,
    },
    Instruction {
        mnemonic: "ADD_M",
        cycles: 7,
        execute: add_m,
    },
    Instruction {
        mnemonic: "ADD_A",
        cycles: 4,
        execute: add_a,
    },
    Instruction {
        mnemonic: "ADC_B",
        cycles: 4,
        execute: adc_b,
    },
    Instruction {
        mnemonic: "ADC_C",
        cycles: 4,
        execute: adc_c,
    },
    Instruction {
        mnemonic: "ADC_D",
        cycles: 4,
        execute: adc_d,
    },
    Instruction {
        mnemonic: "ADC_E",
        cycles: 4,
        execute: adc_e,
    },
    Instruction {
        mnemonic: "ADC_H",
        cycles: 4,
        execute: adc_h,
    },
    Instruction {
        mnemonic: "ADC_L",
        cycles: 4,
        execute: adc_l,
    },
    Instruction {
        mnemonic: "ADC_M",
        cycles: 7,
        execute: adc_m,
    },
    Instruction {
        mnemonic: "ADC_A",
        cycles: 4,
        execute: adc_a,
    },
    Instruction {
        mnemonic: "SUB_B",
        cycles: 4,
        execute: sub_b,
    },
    Instruction {
        mnemonic: "SUB_C",
        cycles: 4,
        execute: sub_c,
    },
    Instruction {
        mnemonic: "SUB_D",
        cycles: 4,
        execute: sub_d,
    },
    Instruction {
        mnemonic: "SUB_E",
        cycles: 4,
        execute: sub_e,
    },
    Instruction {
        mnemonic: "SUB_H",
        cycles: 4,
        execute: sub_h,
    },
    Instruction {
        mnemonic: "SUB_L",
        cycles: 4,
        execute: sub_l,
    },
    Instruction {
        mnemonic: "SUB_M",
        cycles: 7,
        execute: sub_m,
    },
    Instruction {
        mnemonic: "SUB_A",
        cycles: 4,
        execute: sub_a,
    },
    Instruction {
        mnemonic: "SBB_B",
        cycles: 4,
        execute: sbb_b,
    },
    Instruction {
        mnemonic: "SBB_C",
        cycles: 4,
        execute: sbb_c,
    },
    Instruction {
        mnemonic: "SBB_D",
        cycles: 4,
        execute: sbb_d,
    },
    Instruction {
        mnemonic: "SBB_E",
        cycles: 4,
        execute: sbb_e,
    },
    Instruction {
        mnemonic: "SBB_H",
        cycles: 4,
        execute: sbb_h,
    },
    Instruction {
        mnemonic: "SBB_L",
        cycles: 4,
        execute: sbb_l,
    },
    Instruction {
        mnemonic: "SBB_M",
        cycles: 7,
        execute: sbb_m,
    },
    Instruction {
        mnemonic: "SBB_A",
        cycles: 4,
        execute: sbb_a,
    },
    Instruction {
        mnemonic: "ANA_B",
        cycles: 4,
        execute: ana_b,
    },
    Instruction {
        mnemonic: "ANA_C",
        cycles: 4,
        execute: ana_c,
    },
    Instruction {
        mnemonic: "ANA_D",
        cycles: 4,
        execute: ana_d,
    },
    Instruction {
        mnemonic: "ANA_E",
        cycles: 4,
        execute: ana_e,
    },
    Instruction {
        mnemonic: "ANA_H",
        cycles: 4,
        execute: ana_h,
    },
    Instruction {
        mnemonic: "ANA_L",
        cycles: 4,
        execute: ana_l,
    },
    Instruction {
        mnemonic: "ANA_M",
        cycles: 7,
        execute: ana_m,
    },
    Instruction {
        mnemonic: "ANA_A",
        cycles: 4,
        execute: ana_a,
    },
    Instruction {
        mnemonic: "XRA_B",
        cycles: 4,
        execute: xra_b,
    },
    Instruction {
        mnemonic: "XRA_C",
        cycles: 4,
        execute: xra_c,
    },
    Instruction {
        mnemonic: "XRA_D",
        cycles: 4,
        execute: xra_d,
    },
    Instruction {
        mnemonic: "XRA_E",
        cycles: 4,
        execute: xra_e,
    },
    Instruction {
        mnemonic: "XRA_H",
        cycles: 4,
        execute: xra_h,
    },
    Instruction {
        mnemonic: "XRA_L",
        cycles: 4,
        execute: xra_l,
    },
    Instruction {
        mnemonic: "XRA_M",
        cycles: 7,
        execute: xra_m,
    },
    Instruction {
        mnemonic: "XRA_A",
        cycles: 4,
        execute: xra_a,
    },
    Instruction {
        mnemonic: "ORA_B",
        cycles: 4,
        execute: ora_b,
    },
    Instruction {
        mnemonic: "ORA_C",
        cycles: 4,
        execute: ora_c,
    },
    Instruction {
        mnemonic: "ORA_D",
        cycles: 4,
        execute: ora_d,
    },
    Instruction {
        mnemonic: "ORA_E",
        cycles: 4,
        execute: ora_e,
    },
    Instruction {
        mnemonic: "ORA_H",
        cycles: 4,
        execute: ora_h,
    },
    Instruction {
        mnemonic: "ORA_L",
        cycles: 4,
        execute: ora_l,
    },
    Instruction {
        mnemonic: "ORA_M",
        cycles: 7,
        execute: ora_m,
    },
    Instruction {
        mnemonic: "ORA_A",
        cycles: 4,
        execute: ora_a,
    },
    Instruction {
        mnemonic: "CMP_B",
        cycles: 4,
        execute: cmp_b,
    },
    Instruction {
        mnemonic: "CMP_C",
        cycles: 4,
        execute: cmp_c,
    },
    Instruction {
        mnemonic: "CMP_D",
        cycles: 4,
        execute: cmp_d,
    },
    Instruction {
        mnemonic: "CMP_E",
        cycles: 4,
        execute: cmp_e,
    },
    Instruction {
        mnemonic: "CMP_H",
        cycles: 4,
        execute: cmp_h,
    },
    Instruction {
        mnemonic: "CMP_L",
        cycles: 4,
        execute: cmp_l,
    },
    Instruction {
        mnemonic: "CMP_M",
        cycles: 7,
        execute: cmp_m,
    },
    Instruction {
        mnemonic: "CMP_A",
        cycles: 4,
        execute: cmp_a,
    },
    Instruction {
        mnemonic: "RNZ",
        cycles: 5,
        execute: rnz,
    },
    Instruction {
        mnemonic: "POP_B",
        cycles: 10,
        execute: pop_bc,
    },
    Instruction {
        mnemonic: "JNZ",
        cycles: 10,
        execute: jnz,
    },
    Instruction {
        mnemonic: "JMP",
        cycles: 10,
        execute: jmp,
    },
    Instruction {
        mnemonic: "CNZ",
        cycles: 11,
        execute: cnz,
    },
    Instruction {
        mnemonic: "PUSH_B",
        cycles: 11,
        execute: push_bc,
    },
    Instruction {
        mnemonic: "ADI",
        cycles: 7,
        execute: adi,
    },
    Instruction {
        mnemonic: "RST_0",
        cycles: 11,
        execute: rst_0,
    },
    Instruction {
        mnemonic: "RZ",
        cycles: 11,
        execute: rz,
    },
    Instruction {
        mnemonic: "RET",
        cycles: 10,
        execute: ret,
    },
    Instruction {
        mnemonic: "JZ",
        cycles: 10,
        execute: jz,
    },
    Instruction {
        mnemonic: "JMP",
        cycles: 10,
        execute: jmp,
    },
    Instruction {
        mnemonic: "CZ",
        cycles: 11,
        execute: cz,
    },
    Instruction {
        mnemonic: "CALL",
        cycles: 17,
        execute: call,
    },
    Instruction {
        mnemonic: "ACI",
        cycles: 7,
        execute: aci,
    },
    Instruction {
        mnemonic: "RST_1",
        cycles: 11,
        execute: rst_1,
    },
    Instruction {
        mnemonic: "RNC",
        cycles: 5,
        execute: rnc,
    },
    Instruction {
        mnemonic: "POP_D",
        cycles: 10,
        execute: pop_de,
    },
    Instruction {
        mnemonic: "JNC",
        cycles: 10,
        execute: jnc,
    },
    Instruction {
        mnemonic: "OUT",
        cycles: 10,
        execute: out,
    },
    Instruction {
        mnemonic: "CNC",
        cycles: 11,
        execute: cnc,
    },
    Instruction {
        mnemonic: "PUSH_D",
        cycles: 11,
        execute: push_de,
    },
    Instruction {
        mnemonic: "SUI",
        cycles: 7,
        execute: sui,
    },
    Instruction {
        mnemonic: "RST_2",
        cycles: 11,
        execute: rst_2,
    },
    Instruction {
        mnemonic: "RC",
        cycles: 5,
        execute: rc,
    },
    Instruction {
        mnemonic: "RET",
        cycles: 10,
        execute: ret,
    },
    Instruction {
        mnemonic: "JC",
        cycles: 10,
        execute: jc,
    },
    Instruction {
       mnemonic: "IN",
        cycles: 10,
        execute: in_,
    },
    Instruction {
        mnemonic: "CC",
        cycles: 11,
        execute: cc,
    },
    Instruction {
        mnemonic: "CALL",
        cycles: 11,
        execute: call,
    },
    Instruction {
        mnemonic: "SBI",
        cycles: 7,
        execute: sbi,
    },
    Instruction {
        mnemonic: "RST_3",
        cycles: 11,
        execute: rst_3,
    },
    Instruction {
        mnemonic: "RPO",
        cycles: 5,
        execute: rpo,
    },
    Instruction {
        mnemonic: "POP_H",
        cycles: 10,
        execute: pop_hl,
    },
    Instruction {
        mnemonic: "JPO",
        cycles: 10,
        execute: jpo,
    },
    Instruction {
        mnemonic: "XTHL",
        cycles: 18,
        execute: xthl,
    },
    Instruction {
        mnemonic: "CPO",
        cycles: 11,
        execute: cpo,
    },
    Instruction {
        mnemonic: "PUSH_H",
        cycles: 11,
        execute: push_hl,
    },
    Instruction {
        mnemonic: "ANI",
        cycles: 7,
        execute: ani,
    },
    Instruction {
        mnemonic: "RST_4",
        cycles: 11,
        execute: rst_4,
    },
    Instruction {
        mnemonic: "RPE",
        cycles: 5,
        execute: rpe,
    },
    Instruction {
        mnemonic: "PCHL",
        cycles: 5,
        execute: pchl,
    },
    Instruction {
        mnemonic: "JPE",
        cycles: 10,
        execute: jpe,
    },
    Instruction {
        mnemonic: "XCHG",
        cycles: 4,
        execute: xchg,
    },
    Instruction {
        mnemonic: "CPE",
        cycles: 11,
        execute: cpe,
    },
    Instruction {
        mnemonic: "CALL",
        cycles: 17,
        execute: call,
    },
    Instruction {
        mnemonic: "XRI",
        cycles: 7,
        execute: xri,
    },
    Instruction {
        mnemonic: "RST_5",
        cycles: 11,
        execute: rst_5,
    },
    Instruction {
        mnemonic: "RP",
        cycles: 5,
        execute: rp,
    },
    Instruction {
        mnemonic: "POP_PSW",
        cycles: 10,
        execute: pop_psw,
    },
    Instruction {
        mnemonic: "JP",
        cycles: 10,
        execute: jp,
    },
    Instruction {
        mnemonic: "DI",
        cycles: 4,
        execute: di,
    },
    Instruction {
        mnemonic: "CP",
        cycles: 11,
        execute: cp,
    },
    Instruction {
        mnemonic: "PUSH_PSW",
        cycles: 11,
        execute: push_psw,
    },
    Instruction {
        mnemonic: "ORI",
        cycles: 7,
        execute: ori,
    },
    Instruction {
        mnemonic: "RST_6",
        cycles: 11,
        execute: rst_6,
    },
    Instruction {
        mnemonic: "RM",
        cycles: 5,
        execute: rm,
    },
    Instruction {
        mnemonic: "SPHL",
        cycles: 5,
        execute: sphl,
    },
    Instruction {
        mnemonic: "JM",
        cycles: 10,
        execute: jm,
    },
    Instruction {
        mnemonic: "EI",
        cycles: 4,
        execute: ei,
    },
    Instruction {
        mnemonic: "CM",
        cycles: 11,
        execute: cm,
    },
    Instruction {
        mnemonic: "CALL",
        cycles: 17,
        execute: call,
    },
    Instruction {
        mnemonic: "CPI",
        cycles: 7,
        execute: cpi,
    },
    Instruction {
        mnemonic: "RST_7",
        cycles: 11,
        execute: rst_7,
    }
];