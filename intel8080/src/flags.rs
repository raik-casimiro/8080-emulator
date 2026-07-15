const FLAG_S: u8 = 0x80;
const FLAG_Z: u8 = 0x40;
const FLAG_AC: u8 = 0x10;
const FLAG_P: u8 = 0x04;
const FLAG_C: u8 = 0x01;

pub struct Flags {
    pub zero: bool,
    pub sign: bool,
    pub auxiliary_carry: bool,
    pub parity: bool,
    pub carry: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            zero: false,
            sign: false,
            auxiliary_carry: false,
            parity: false,
            carry: false,
        }
    }

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
        (if self.sign { FLAG_S } else { 0 })
        | (if self.zero { FLAG_Z } else { 0 })
        | (if self.auxiliary_carry { FLAG_AC } else { 0 })
        | (if self.parity { FLAG_P } else { 0 })
        | (if self.carry { FLAG_C } else { 0 })
        | 0x02
    }

    pub fn unpack(&mut self, value: u8) {
        self.sign = (value & 0x80) != 0;
        self.zero = (value & 0x40) != 0;
        self.auxiliary_carry = (value & 0x10) != 0;
        self.parity = (value & 0x04) != 0;
        self.carry = (value & 0x01) != 0;
    }
}
