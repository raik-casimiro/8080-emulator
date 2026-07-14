use crate::memory::Memory;
use crate::opcodes::OPCODES;

pub struct IO {
    port: [u8; 0xFF],
}

pub struct Flags {
    pub zero: bool,
    pub sign: bool,
    pub auxiliary_carry: bool,
    pub parity: bool,
    pub carry: bool,
}

pub struct CpuState {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: Flags,
}

pub struct Cpu {
    pub state: CpuState,
    pub memory: Memory,
    pub io: IO,
    pub pc: u16,
    pub sp: u16,
    pub halted: bool,
    pub enable_interrupts: bool,
}

impl Flags {
    pub fn update_szp(&mut self, value: u8) {
        self.zero = value == 0;
        self.sign = (value & 0x80) != 0;
        self.parity = value.count_ones() % 2 == 0;
    }

    pub fn update_auxiliary_carry_add(&mut self, a: u8, b: u8) {
        self.auxiliary_carry = ((a & 0x0F) + (b & 0x0F)) > 0x0F;
    }

    pub fn update_auxiliary_carry_sub(&mut self, a: u8, b: u8) {
        self.auxiliary_carry = (a & 0x0F) < (b & 0x0F);
    }

    pub fn pack(&self) -> u8 {
        let mut value = 0;

        if self.sign {
            value |= 0x80;
        }

        if self.zero {
            value |= 0x40;
        }

        if self.auxiliary_carry {
            value |= 0x10;
        }

        if self.parity {
            value |= 0x04;
        }

        value |= 0x02;

        if self.carry {
            value |= 0x01;
        }

        value
    }

    pub fn unpack(&mut self, value: u8) {
        self.sign = (value & 0x80) != 0;
        self.zero = (value & 0x40) != 0;
        self.auxiliary_carry = (value & 0x10) != 0;
        self.parity = (value & 0x04) != 0;
        self.carry = (value & 0x01) != 0;
    }
}

impl IO {
    pub fn new() -> IO {
        IO {
            port: [0; 0xFF],
        }
    }

    pub fn read(&self, port: u8) -> u8 {
        self.port[port as usize]
    }

    pub fn write(&mut self, port: u8, value: u8) {
        self.port[port as usize] = value;
    }
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: Flags {
                carry: false,
                parity: false,
                sign: false,
                zero: false,
                auxiliary_carry: false
            }
        }
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn add(&mut self, value: u8) {
        self.flags.update_auxiliary_carry_add(self.a, value);

        let (result, overflow) = self.a.overflowing_add(value);
        self.flags.update_szp(result);
        self.flags.carry = overflow;

        self.a = result;
    }

    pub fn adc(&mut self, value: u8) {
        let old_carry = self.flags.carry as u8;

        self.flags.auxiliary_carry = ((self.a & 0x0F) + (value & 0x0F) + old_carry) > 0x0F;

        let (result, carry1) = self.a.overflowing_add(value);
        let (result, carry2) = result.overflowing_add(old_carry);

        self.a = result;
        self.flags.update_szp(result);
        self.flags.carry = carry1 || carry2;
    }

    pub fn sub(&mut self, value: u8) {
        self.flags.update_auxiliary_carry_sub(self.a, value);

        let (result, borrow) = self.a.overflowing_sub(value);

        self.a = result;

        self.flags.carry = borrow;
        self.flags.update_szp(result);
    }

    pub fn sbb(&mut self, value: u8) {
        let old_carry = self.flags.carry as u8;

        self.flags.auxiliary_carry = (self.a & 0x0F) < ((value & 0x0F) + old_carry);

        let (result, borrow1) = self.a.overflowing_sub(value);
        let (result, borrow2) = result.overflowing_sub(old_carry);

        self.a = result;
        self.flags.carry = borrow1 || borrow2;
        self.flags.update_szp(result);
    }

    pub fn ana(&mut self, value: u8) {
        self.flags.auxiliary_carry = true;
        self.flags.carry = false;

        self.a &= value;
        self.flags.update_szp(self.a);
    }

    pub fn xra(&mut self, value: u8) {
        self.flags.auxiliary_carry = false;
        self.flags.carry = false;

        self.a ^= value;
        self.flags.update_szp(self.a);
    }

    pub fn ora(&mut self, value: u8) {
        self.flags.auxiliary_carry = false;
        self.flags.carry = false;

        self.a |= value;
        self.flags.update_szp(self.a);
    }

    pub fn cmp(&mut self, value: u8) {
        self.flags.update_auxiliary_carry_sub(self.a, value);

        let (result, borrow) = self.a.overflowing_sub(value);
        self.flags.carry = borrow;
        self.flags.update_szp(result);
    }
}


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state: CpuState::new(),
            memory: Memory::new(),
            io: IO::new(),
            pc: 0,
            sp: 0x2400,
            halted: false,
            enable_interrupts: false,
        }
    }

    pub fn rst(&mut self, address: u16) {
        self.push_word(self.pc);
        self.pc = address;
    }

    pub fn push_word(&mut self, value: u16) {
        self.sp -= 2;

        self.memory.write8(self.sp, value as u8);
        self.memory.write8(self.sp + 1, (value >> 8) as u8);
    }

    pub fn pop_word(&mut self) -> u16 {
        let lo = self.memory.read8(self.sp) as u16;
        let hi = self.memory.read8(self.sp + 1) as u16;
        self.sp += 2;

        (hi << 8) | lo
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory.read8(self.pc);
        self.pc += 1;

        byte
    }

    pub fn fetch_word(&mut self) -> u16 {
        let lo = self.fetch_byte() as u16;
        let hi = self.fetch_byte() as u16;

        (hi << 8) | lo
    }

    pub fn read_m(&self) -> u8 {
        self.memory.read8(self.state.hl())
    }

    pub fn write_m(&mut self, value: u8) {
        self.memory.write8(self.state.hl(), value);
    }

    pub fn call_if(&mut self, address: u16, condition: bool) {
        if !condition {
            return;
        }

        self.push_word(self.pc);
        self.pc = address;
    }

    pub fn jump_if(&mut self, address: u16, condition: bool) {
        if !condition {
            return;
        }

        self.pc = address;
    }

    pub fn return_if(&mut self, condition: bool) {
        if !condition {
            return;
        }

        self.pc = self.pop_word();
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

    pub fn step(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        let instruction = &OPCODES[opcode as usize];

        (instruction.execute)(self);

        self.debug(opcode);
        
        instruction.cycles
    }
}
