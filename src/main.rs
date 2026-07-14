use crate::cpu::Cpu;
mod opcodes;
mod instructions;
mod memory;
mod cpu;

pub fn main() {
    let mut cpu = Cpu::new();

    let rom = std::fs::read("rom/invaders.rom").unwrap();
    cpu.memory.load(&rom);

    loop {
        if cpu.halted {
            break;
        }

        cpu.step();
    }
}

