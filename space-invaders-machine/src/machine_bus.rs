use intel8080::Bus;
use crate::audio::Audio;
use crate::inputs::Inputs;
use crate::memory::Memory;
use crate::shift_register::ShiftRegister;
use crate::video::Video;

pub struct MachineBus {
    pub memory: Memory,
    pub inputs: Inputs,
    pub video: Video,
    pub audio: Audio,
    pub shift_register: ShiftRegister,
}

impl MachineBus {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            inputs: Inputs::new(),
            video: Video::new(),
            audio: Audio::new(),
            shift_register: ShiftRegister::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory.load(rom);
    }
}

impl Bus for MachineBus {
    fn read8(&mut self, addr: u16) -> u8 {
        self.memory.read8(addr)
    }

    fn write8(&mut self, addr: u16, value: u8) {
        self.memory.write8(addr, value)
    }

    fn input(&mut self, port: u8) -> u8 {
        match port {
            1 => self.inputs.port1(),
            2 => self.inputs.port2(),
            3 => self.shift_register.read(),
            _ => 0,
        }
    }

    fn output(&mut self, port: u8, value: u8) {
        match port {
            2 => self.shift_register.set_offset(value),
            3 => self.audio.write_port3(value),
            4 => self.shift_register.write(value),
            5 => self.audio.write_port5(value),
            _ => {}
        }
    }
}