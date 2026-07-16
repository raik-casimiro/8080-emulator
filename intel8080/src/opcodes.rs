use crate::cpu::{Cpu, CpuContext};
use crate::instructions::*;

pub struct Instruction {
    pub mnemonic: &'static str,
    pub execute: fn(&mut Cpu, ctx: &mut CpuContext) -> u8,
}

pub static OPCODES: [Instruction; 256] = [
    Instruction {
        mnemonic: "NOP",
        execute: nop,
    },
    Instruction {
        mnemonic: "LXI B",
        execute: lxi_b,
    },
    Instruction {
        mnemonic: "STAX B",
        execute: stax_b,
    },
    Instruction {
        mnemonic: "INX B",
        execute: inx_b,
    },
    Instruction {
        mnemonic: "INR B",
        execute: inr_b,
    },
    Instruction {
        mnemonic: "DCR B",
        execute: dcr_b,
    },
    Instruction {
        mnemonic: "MVI B",
        execute: mvi_b,
    },
    Instruction {
        mnemonic: "RLC",
        execute: rlc,
    },
    Instruction {
        mnemonic: "NOP",
        execute: nop,
    },
    Instruction {
        mnemonic: "DAD_B",
        execute: dad_b,
    },
    Instruction {
        mnemonic: "LDAX_B",
        execute: ldax_b,
    },
    Instruction {
        mnemonic: "DCX_B",
        execute: dcx_b,
    },
    Instruction {
        mnemonic: "INR_C",
        execute: inr_c,
    },
    Instruction {
        mnemonic: "DCR_C",
        execute: dcr_c,
    },
    Instruction {
        mnemonic: "MVI_C",
        execute: mvi_c,
    },
    Instruction {
        mnemonic: "RRC",
        execute: rrc,
    },
    Instruction {
        mnemonic: "NOP",
        execute: nop,
    },
    Instruction {
        mnemonic: "LXI_D",
        execute: lxi_d,
    },
    Instruction {
        mnemonic: "STAX_D",
        execute: stax_d,
    },
    Instruction {
        mnemonic: "INX_D",
        execute: inx_d,
    },
    Instruction {
        mnemonic: "INR_D",
        execute: inr_d,
    },
    Instruction {
        mnemonic: "DCR_D",
        execute: dcr_d,
    },
    Instruction {
        mnemonic: "MVI_D",
        execute: mvi_d,
    },
    Instruction {
        mnemonic: "RAL",
        execute: ral,
    },
    Instruction {
        mnemonic: "NOP",
        execute: nop,
    },
    Instruction {
        mnemonic: "DAD_D",
        execute: dad_d,
    },
    Instruction {
        mnemonic: "LDAX_D",
        execute: ldax_d,
    },
    Instruction {
        mnemonic: "DCX_D",
        execute: dcx_d,
    },
    Instruction {
        mnemonic: "INR_E",
        execute: inr_e,
    },
    Instruction {
        mnemonic: "DCR_E",
        execute: dcr_e,
    },
    Instruction {
        mnemonic: "MVI_E",
        execute: mvi_e,
    },
    Instruction {
        mnemonic: "RAR",
        execute: rar,
    },
    Instruction {
        mnemonic: "NOP",
        execute: nop,
    },
    Instruction {
        mnemonic: "LXI_H",
        execute: lxi_h,
    },
    Instruction {
        mnemonic: "SHLD",
        execute: shld,
    },
    Instruction {
        mnemonic: "INX_H",
        execute: inx_h,
    },
    Instruction {
        mnemonic: "INR_H",
        execute: inr_h,
    },
    Instruction {
        mnemonic: "DCR_H",
        execute: dcr_h,
    },
    Instruction {
        mnemonic: "MVI_H",
        execute: mvi_h,
    },
    Instruction {
        mnemonic: "DAA",
        execute: daa,
    },
    Instruction {
        mnemonic: "NOP",
        execute: nop,
    },
    Instruction {
        mnemonic: "DAD_H",
        execute: dad_h,
    },
    Instruction {
        mnemonic: "LHLD",
        execute: lhld,
    },
    Instruction {
        mnemonic: "DCX_H",
        execute: dcx_h,
    },
    Instruction {
        mnemonic: "INR_L",
        execute: inr_l,
    },
    Instruction {
        mnemonic: "DCR_L",
        execute: dcr_l,
    },
    Instruction {
        mnemonic: "MVI_L",
        execute: mvi_l,
    },
    Instruction {
        mnemonic: "CMA",
        execute: cma,
    },
    Instruction {
        mnemonic: "NOP",
        execute: nop,
    },
    Instruction {
        mnemonic: "LXI_SP",
        execute: lxi_sp,
    },
    Instruction {
        mnemonic: "STA",
        execute: sta,
    },
    Instruction {
        mnemonic: "INX_SP",
        execute: inx_sp,
    },
    Instruction {
        mnemonic: "INR_M",
        execute: inr_m,
    },
    Instruction {
        mnemonic: "DCR_M",
        execute: dcr_m,
    },
    Instruction {
        mnemonic: "MVI_M",
        execute: mvi_m,
    },
    Instruction {
        mnemonic: "STC",
        execute: stc,
    },
    Instruction {
        mnemonic: "NOP",
        execute: nop,
    },
    Instruction {
        mnemonic: "DAD_SP",
        execute: dad_sp,
    },
    Instruction {
        mnemonic: "LDA",
        execute: lda,
    },
    Instruction {
        mnemonic: "DCX_SP",
        execute: dcx_sp,
    },
    Instruction {
        mnemonic: "INR_A",
        execute: inr_a,
    },
    Instruction {
        mnemonic: "DCR_A",
        execute: dcr_a,
    },
    Instruction {
        mnemonic: "MVI_A",
        execute: mvi_a,
    },
    Instruction {
        mnemonic: "CMC",
        execute: cmc,
    },
    Instruction {
        mnemonic: "MOV_B_B",
        execute: mov_bb,
    },
    Instruction {
        mnemonic: "MOV_B_C",
        execute: mov_bc,
    },
    Instruction {
        mnemonic: "MOV_B_D",
        execute: mov_bd,
    },
    Instruction {
        mnemonic: "MOV_B_E",
        execute: mov_be,
    },
    Instruction {
        mnemonic: "MOV_B_H",
        execute: mov_bh,
    },
    Instruction {
        mnemonic: "MOV_B_L",
        execute: mov_bl,
    },
    Instruction {
        mnemonic: "MOV_B_M",
        execute: mov_bm,
    },
    Instruction {
        mnemonic: "MOV_B_A",
        execute: mov_ba,
    },
    Instruction {
        mnemonic: "MOV_C_B",
        execute: mov_cb,
    },
    Instruction {
        mnemonic: "MOV_C_C",
        execute: mov_cc,
    },
    Instruction {
        mnemonic: "MOV_C_D",
        execute: mov_cd,
    },
    Instruction {
        mnemonic: "MOV_C_E",
        execute: mov_ce,
    },
    Instruction {
        mnemonic: "MOV_C_H",
        execute: mov_ch,
    },
    Instruction {
        mnemonic: "MOV_C_L",
        execute: mov_cl,
    },
    Instruction {
        mnemonic: "MOV_C_M",
        execute: mov_cm,
    },
    Instruction {
        mnemonic: "MOV_C_A",
        execute: mov_ca,
    },
    Instruction {
        mnemonic: "MOV_D_B",
        execute: mov_db,
    },
    Instruction {
        mnemonic: "MOV_D_C",
        execute: mov_dc,
    },
    Instruction {
        mnemonic: "MOV_D_D",
        execute: mov_dd,
    },
    Instruction {
        mnemonic: "MOV_D_E",
        execute: mov_de,
    },
    Instruction {
        mnemonic: "MOV_D_H",
        execute: mov_dh,
    },
    Instruction {
        mnemonic: "MOV_D_L",
        execute: mov_dl,
    },
    Instruction {
        mnemonic: "MOV_D_M",
        execute: mov_dm,
    },
    Instruction {
        mnemonic: "MOV_D_A",
        execute: mov_da,
    },
    Instruction {
        mnemonic: "MOV_E_B",
        execute: mov_eb,
    },
    Instruction {
        mnemonic: "MOV_E_C",
        execute: mov_ec,
    },
    Instruction {
        mnemonic: "MOV_E_D",
        execute: mov_ed,
    },
    Instruction {
        mnemonic: "MOV_E_E",
        execute: mov_ee,
    },
    Instruction {
        mnemonic: "MOV_E_h",
        execute: mov_eh,
    },
    Instruction {
        mnemonic: "MOV_E_L",
        execute: mov_el,
    },
    Instruction {
        mnemonic: "MOV_E_M",
        execute: mov_em,
    },
    Instruction {
        mnemonic: "MOV_E_A",
        execute: mov_ea,
    },
    Instruction {
        mnemonic: "MOV_H_B",
        execute: mov_hb,
    },
    Instruction {
        mnemonic: "MOV_H_C",
        execute: mov_hc,
    },
    Instruction {
        mnemonic: "MOV_H_D",
        execute: mov_hd,
    },
    Instruction {
        mnemonic: "MOV_H_E",
        execute: mov_he,
    },
    Instruction {
        mnemonic: "MOV_H_H",
        execute: mov_hh,
    },
    Instruction {
        mnemonic: "MOV_H_L",
        execute: mov_hl,
    },
    Instruction {
        mnemonic: "MOV_H_M",
        execute: mov_hm,
    },
    Instruction {
        mnemonic: "MOV_H_A",
        execute: mov_ha,
    },
    Instruction {
        mnemonic: "MOV_L_B",
        execute: mov_lb,
    },
    Instruction {
        mnemonic: "MOV_L_C",
        execute: mov_lc,
    },
    Instruction {
        mnemonic: "MOV_L_D",
        execute: mov_ld,
    },
    Instruction {
        mnemonic: "MOV_L_E",
        execute: mov_le,
    },
    Instruction {
        mnemonic: "MOV_L_H",
        execute: mov_lh,
    },
    Instruction {
        mnemonic: "MOV_L_L",
        execute: mov_ll,
    },
    Instruction {
        mnemonic: "MOV_L_M",
        execute: mov_lm,
    },
    Instruction {
        mnemonic: "MOV_L_A",
        execute: mov_la,
    },
    Instruction {
        mnemonic: "MOV_M_B",
        execute: mov_mb,
    },
    Instruction {
        mnemonic: "MOV_M_C",
        execute: mov_mc,
    },
    Instruction {
        mnemonic: "MOV_M_D",
        execute: mov_md,
    },
    Instruction {
        mnemonic: "MOV_M_E",
        execute: mov_me,
    },
    Instruction {
        mnemonic: "MOV_M_H",
        execute: mov_mh,
    },
    Instruction {
        mnemonic: "MOV_M_L",
        execute: mov_ml,
    },
    Instruction {
        mnemonic: "HLT",
        execute: hlt,
    },
    Instruction {
        mnemonic: "MOV_M_A",
        execute: mov_ma,
    },
    Instruction {
        mnemonic: "MOV_A_B",
        execute: mov_ab,
    },
    Instruction {
        mnemonic: "MOV_A_C",
        execute: mov_ac,
    },
    Instruction {
        mnemonic: "MOV_A_D",
        execute: mov_ad,
    },
    Instruction {
        mnemonic: "MOV_A_E",
        execute: mov_ae,
    },
    Instruction {
        mnemonic: "MOV_A_h",
        execute: mov_ah,
    },
    Instruction {
        mnemonic: "MOV_A_L",
        execute: mov_al,
    },
    Instruction {
        mnemonic: "MOV_A_M",
        execute: mov_am,
    },
    Instruction {
        mnemonic: "MOV_A_A",
        execute: mov_aa,
    },
    Instruction {
        mnemonic: "ADD_B",
        execute: add_b,
    },
    Instruction {
        mnemonic: "ADD_C",
        execute: add_c,
    },
    Instruction {
        mnemonic: "ADD_D",
        execute: add_d,
    },
    Instruction {
        mnemonic: "ADD_E",
        execute: add_e,
    },
    Instruction {
        mnemonic: "ADD_H",
        execute: add_h,
    },
    Instruction {
        mnemonic: "ADD_L",
        execute: add_l,
    },
    Instruction {
        mnemonic: "ADD_M",
        execute: add_m,
    },
    Instruction {
        mnemonic: "ADD_A",
        execute: add_a,
    },
    Instruction {
        mnemonic: "ADC_B",
        execute: adc_b,
    },
    Instruction {
        mnemonic: "ADC_C",
        execute: adc_c,
    },
    Instruction {
        mnemonic: "ADC_D",
        execute: adc_d,
    },
    Instruction {
        mnemonic: "ADC_E",
        execute: adc_e,
    },
    Instruction {
        mnemonic: "ADC_H",
        execute: adc_h,
    },
    Instruction {
        mnemonic: "ADC_L",
        execute: adc_l,
    },
    Instruction {
        mnemonic: "ADC_M",
        execute: adc_m,
    },
    Instruction {
        mnemonic: "ADC_A",
        execute: adc_a,
    },
    Instruction {
        mnemonic: "SUB_B",
        execute: sub_b,
    },
    Instruction {
        mnemonic: "SUB_C",
        execute: sub_c,
    },
    Instruction {
        mnemonic: "SUB_D",
        execute: sub_d,
    },
    Instruction {
        mnemonic: "SUB_E",
        execute: sub_e,
    },
    Instruction {
        mnemonic: "SUB_H",
        execute: sub_h,
    },
    Instruction {
        mnemonic: "SUB_L",
        execute: sub_l,
    },
    Instruction {
        mnemonic: "SUB_M",
        execute: sub_m,
    },
    Instruction {
        mnemonic: "SUB_A",
        execute: sub_a,
    },
    Instruction {
        mnemonic: "SBB_B",
        execute: sbb_b,
    },
    Instruction {
        mnemonic: "SBB_C",
        execute: sbb_c,
    },
    Instruction {
        mnemonic: "SBB_D",
        execute: sbb_d,
    },
    Instruction {
        mnemonic: "SBB_E",
        execute: sbb_e,
    },
    Instruction {
        mnemonic: "SBB_H",
        execute: sbb_h,
    },
    Instruction {
        mnemonic: "SBB_L",
        execute: sbb_l,
    },
    Instruction {
        mnemonic: "SBB_M",
        execute: sbb_m,
    },
    Instruction {
        mnemonic: "SBB_A",
        execute: sbb_a,
    },
    Instruction {
        mnemonic: "ANA_B",
        execute: ana_b,
    },
    Instruction {
        mnemonic: "ANA_C",
        execute: ana_c,
    },
    Instruction {
        mnemonic: "ANA_D",
        execute: ana_d,
    },
    Instruction {
        mnemonic: "ANA_E",
        execute: ana_e,
    },
    Instruction {
        mnemonic: "ANA_H",
        execute: ana_h,
    },
    Instruction {
        mnemonic: "ANA_L",
        execute: ana_l,
    },
    Instruction {
        mnemonic: "ANA_M",
        execute: ana_m,
    },
    Instruction {
        mnemonic: "ANA_A",
        execute: ana_a,
    },
    Instruction {
        mnemonic: "XRA_B",
        execute: xra_b,
    },
    Instruction {
        mnemonic: "XRA_C",
        execute: xra_c,
    },
    Instruction {
        mnemonic: "XRA_D",
        execute: xra_d,
    },
    Instruction {
        mnemonic: "XRA_E",
        execute: xra_e,
    },
    Instruction {
        mnemonic: "XRA_H",
        execute: xra_h,
    },
    Instruction {
        mnemonic: "XRA_L",
        execute: xra_l,
    },
    Instruction {
        mnemonic: "XRA_M",
        execute: xra_m,
    },
    Instruction {
        mnemonic: "XRA_A",
        execute: xra_a,
    },
    Instruction {
        mnemonic: "ORA_B",
        execute: ora_b,
    },
    Instruction {
        mnemonic: "ORA_C",
        execute: ora_c,
    },
    Instruction {
        mnemonic: "ORA_D",
        execute: ora_d,
    },
    Instruction {
        mnemonic: "ORA_E",
        execute: ora_e,
    },
    Instruction {
        mnemonic: "ORA_H",
        execute: ora_h,
    },
    Instruction {
        mnemonic: "ORA_L",
        execute: ora_l,
    },
    Instruction {
        mnemonic: "ORA_M",
        execute: ora_m,
    },
    Instruction {
        mnemonic: "ORA_A",
        execute: ora_a,
    },
    Instruction {
        mnemonic: "CMP_B",
        execute: cmp_b,
    },
    Instruction {
        mnemonic: "CMP_C",
        execute: cmp_c,
    },
    Instruction {
        mnemonic: "CMP_D",
        execute: cmp_d,
    },
    Instruction {
        mnemonic: "CMP_E",
        execute: cmp_e,
    },
    Instruction {
        mnemonic: "CMP_H",
        execute: cmp_h,
    },
    Instruction {
        mnemonic: "CMP_L",
        execute: cmp_l,
    },
    Instruction {
        mnemonic: "CMP_M",
        execute: cmp_m,
    },
    Instruction {
        mnemonic: "CMP_A",
        execute: cmp_a,
    },
    Instruction {
        mnemonic: "RNZ",
        execute: rnz,
    },
    Instruction {
        mnemonic: "POP_B",
        execute: pop_bc,
    },
    Instruction {
        mnemonic: "JNZ",
        execute: jnz,
    },
    Instruction {
        mnemonic: "JMP",
        execute: jmp,
    },
    Instruction {
        mnemonic: "CNZ",
        execute: cnz,
    },
    Instruction {
        mnemonic: "PUSH_B",
        execute: push_bc,
    },
    Instruction {
        mnemonic: "ADI",
        execute: adi,
    },
    Instruction {
        mnemonic: "RST_0",
        execute: rst_0,
    },
    Instruction {
        mnemonic: "RZ",
        execute: rz,
    },
    Instruction {
        mnemonic: "RET",
        execute: ret,
    },
    Instruction {
        mnemonic: "JZ",
        execute: jz,
    },
    Instruction {
        mnemonic: "JMP",
        execute: jmp,
    },
    Instruction {
        mnemonic: "CZ",
        execute: cz,
    },
    Instruction {
        mnemonic: "CALL",
        execute: call,
    },
    Instruction {
        mnemonic: "ACI",
        execute: aci,
    },
    Instruction {
        mnemonic: "RST_1",
        execute: rst_1,
    },
    Instruction {
        mnemonic: "RNC",
        execute: rnc,
    },
    Instruction {
        mnemonic: "POP_D",
        execute: pop_de,
    },
    Instruction {
        mnemonic: "JNC",
        execute: jnc,
    },
    Instruction {
        mnemonic: "OUT",
        execute: out,
    },
    Instruction {
        mnemonic: "CNC",
        execute: cnc,
    },
    Instruction {
        mnemonic: "PUSH_D",
        execute: push_de,
    },
    Instruction {
        mnemonic: "SUI",
        execute: sui,
    },
    Instruction {
        mnemonic: "RST_2",
        execute: rst_2,
    },
    Instruction {
        mnemonic: "RC",
        execute: rc,
    },
    Instruction {
        mnemonic: "RET",
        execute: ret,
    },
    Instruction {
        mnemonic: "JC",
        execute: jc,
    },
    Instruction {
       mnemonic: "IN",
        execute: in_,
    },
    Instruction {
        mnemonic: "CC",
        execute: cc,
    },
    Instruction {
        mnemonic: "CALL",
        execute: call,
    },
    Instruction {
        mnemonic: "SBI",
        execute: sbi,
    },
    Instruction {
        mnemonic: "RST_3",
        execute: rst_3,
    },
    Instruction {
        mnemonic: "RPO",
        execute: rpo,
    },
    Instruction {
        mnemonic: "POP_H",
        execute: pop_hl,
    },
    Instruction {
        mnemonic: "JPO",
        execute: jpo,
    },
    Instruction {
        mnemonic: "XTHL",
        execute: xthl,
    },
    Instruction {
        mnemonic: "CPO",
        execute: cpo,
    },
    Instruction {
        mnemonic: "PUSH_H",
        execute: push_hl,
    },
    Instruction {
        mnemonic: "ANI",
        execute: ani,
    },
    Instruction {
        mnemonic: "RST_4",
        execute: rst_4,
    },
    Instruction {
        mnemonic: "RPE",
        execute: rpe,
    },
    Instruction {
        mnemonic: "PCHL",
        execute: pchl,
    },
    Instruction {
        mnemonic: "JPE",
        execute: jpe,
    },
    Instruction {
        mnemonic: "XCHG",
        execute: xchg,
    },
    Instruction {
        mnemonic: "CPE",
        execute: cpe,
    },
    Instruction {
        mnemonic: "CALL",
        execute: call,
    },
    Instruction {
        mnemonic: "XRI",
        execute: xri,
    },
    Instruction {
        mnemonic: "RST_5",
        execute: rst_5,
    },
    Instruction {
        mnemonic: "RP",
        execute: rp,
    },
    Instruction {
        mnemonic: "POP_PSW",
        execute: pop_psw,
    },
    Instruction {
        mnemonic: "JP",
        execute: jp,
    },
    Instruction {
        mnemonic: "DI",
        execute: di,
    },
    Instruction {
        mnemonic: "CP",
        execute: cp,
    },
    Instruction {
        mnemonic: "PUSH_PSW",
        execute: push_psw,
    },
    Instruction {
        mnemonic: "ORI",
        execute: ori,
    },
    Instruction {
        mnemonic: "RST_6",
        execute: rst_6,
    },
    Instruction {
        mnemonic: "RM",
        execute: rm,
    },
    Instruction {
        mnemonic: "SPHL",
        execute: sphl,
    },
    Instruction {
        mnemonic: "JM",
        execute: jm,
    },
    Instruction {
        mnemonic: "EI",
        execute: ei,
    },
    Instruction {
        mnemonic: "CM",
        execute: cm,
    },
    Instruction {
        mnemonic: "CALL",
        execute: call,
    },
    Instruction {
        mnemonic: "CPI",
        execute: cpi,
    },
    Instruction {
        mnemonic: "RST_7",
        execute: rst_7,
    }
];