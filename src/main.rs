use minifb::{Window, WindowOptions};
use space_invaders_machine::machine::Machine;
use space_invaders_machine::video::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn main() {

    let mut machine = Machine::new();
    let rom = std::fs::read("space-invaders-machine/rom/invaders.rom").unwrap();

    machine.bus.load_rom(&rom);
    machine.run();
}


