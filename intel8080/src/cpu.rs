use crate::Bus;
use crate::opcodes::OPCODES;
use crate::state::CpuState;

pub struct CpuContext<'a> {
    pub bus: &'a mut dyn Bus,
}

pub struct Cpu {
    pub state: CpuState,
    pub pc: u16,
    pub sp: u16,
    pub halted: bool,
    pub enable_interrupts: bool,
    pub debug_enabled: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            state: CpuState::new(),
            pc: 0,
            sp: 0x2400,
            halted: false,
            enable_interrupts: false,
            debug_enabled: false,
        }
    }

    pub fn call_if(&mut self, ctx: &mut CpuContext, address: u16, condition: bool) -> bool {
        if !condition {
            return false;
        }

        self.push_word(ctx, self.pc);
        self.pc = address;
        true
    }

    pub fn jump_if(&mut self, address: u16, condition: bool) -> bool {
        if !condition {
            return false;
        }

        self.pc = address;
        true
    }

    pub fn return_if(&mut self, ctx: &mut CpuContext, condition: bool) -> bool {
        if !condition {
            return false;
        }

        self.pc = self.pop_word(ctx);
        true
    }

    pub fn debug(&self, opcode: u8) {
        let instruction = &OPCODES[opcode as usize];

        println!(
            "OPCODE: {:02X} | INSTRUCTION: {}",
            opcode,
            instruction.mnemonic
        );

        println!(
            "A: {:02X}  B: {:02X}  C: {:02X}  D: {:02X}  E: {:02X}  H: {:02X}  L: {:02X}",
            self.state.a,
            self.state.b,
            self.state.c,
            self.state.d,
            self.state.e,
            self.state.h,
            self.state.l,
        );

        println!(
            "FLAGS: S={} Z={} P={} CY={} AC={}",
            self.state.flags.sign,
            self.state.flags.zero,
            self.state.flags.parity,
            self.state.flags.carry,
            self.state.flags.auxiliary_carry,
        );

        println!(
            "STATE: SP={:04X} HALTED={} PC={:04X}",
            self.sp,
            self.halted,
            self.pc,
        );

        println!();
    }

    pub fn step(&mut self, ctx: &mut CpuContext) -> u8 {
        let opcode = self.fetch_byte(ctx);
        let instruction = &OPCODES[opcode as usize];

        let cycles = (instruction.execute)(self, ctx);

        if self.debug_enabled {
            self.debug(opcode);
        }

        cycles
    }
}
