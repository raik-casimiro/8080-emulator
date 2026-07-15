pub struct Inputs {
    pub coin: bool,
    pub start1: bool,
    pub start2: bool,

    pub p1_shoot: bool,
    pub p1_left: bool,
    pub p1_right: bool,

    pub p2_shoot: bool,
    pub p2_left: bool,
    pub p2_right: bool,

    pub dip_switches: u8,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            coin: false,
            start1: false,
            start2: false,

            p1_shoot: false,
            p1_left: false,
            p1_right: false,

            p2_shoot: false,
            p2_left: false,
            p2_right: false,

            dip_switches: 0,
        }
    }

    pub fn port1(&self) -> u8 {
        let mut value = 0;

        if self.coin      { value |= 1 << 0; }
        if self.start2    { value |= 1 << 1; }
        if self.start1    { value |= 1 << 2; }

        value |= 1 << 3;

        if self.p1_shoot  { value |= 1 << 4; }
        if self.p1_left   { value |= 1 << 5; }
        if self.p1_right  { value |= 1 << 6; }

        value
    }

    pub fn port2(&self) -> u8 {
        let mut value = self.dip_switches;

        if self.p2_shoot  { value |= 1 << 4; }
        if self.p2_left   { value |= 1 << 5; }
        if self.p2_right  { value |= 1 << 6; }

        value
    }
}
