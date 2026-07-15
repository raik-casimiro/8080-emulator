use intel8080::{Bus, Cpu};
use intel8080::cpu::CpuContext;
use crate::machine_bus::MachineBus;

pub struct Machine {
    pub cpu: Cpu,
    pub bus: MachineBus,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: MachineBus::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.bus.memory.load(rom);
    }

    pub fn step(&mut self) {
        let ctx = &mut CpuContext {
            bus: &mut self.bus,
            cycles: 0
        };

        loop {
            if self.cpu.halted {
                break;
            }

            self.cpu.step(ctx);
        }
    }
}