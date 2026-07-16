use intel8080::Cpu;
use intel8080::cpu::CpuContext;
use crate::machine_bus::MachineBus;
use minifb::{Window, WindowOptions};
use crate::video::{SCREEN_HEIGHT, SCREEN_WIDTH};

const CYCLES_PER_FRAME: u64 = 2_000_000;

pub struct Machine {
    pub cpu: Cpu,
    pub bus: MachineBus,
    pub window: Window
}

impl Machine {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: MachineBus::new(),
            window: Window::new(
                "Space Invaders",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                WindowOptions::default(),
            ).unwrap()
        }
    }

    pub fn request_interrupt(&mut self, rst: u8) {
        let ctx = &mut CpuContext {
            bus: &mut self.bus
        };

        self.cpu.interrupt(ctx, rst);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.bus.memory.load(rom);
    }


    pub fn handle_input(&mut self) {
        self.cpu.debug_enabled = self.window.is_key_down(minifb::Key::F1);
        self.cpu.halted = self.window.is_key_down(minifb::Key::Escape);
        self.bus.inputs.coin = self.window.is_key_down(minifb::Key::Space);
        self.bus.inputs.start1 = self.window.is_key_down(minifb::Key::Enter);
        self.bus.inputs.p1_left = self.window.is_key_down(minifb::Key::Left);
        self.bus.inputs.p1_right = self.window.is_key_down(minifb::Key::Right);
        self.bus.inputs.p1_shoot = self.window.is_key_down(minifb::Key::Up);
    }

    pub fn run_one_frame(&mut self) {
        let mut total_cycles ;

        while self.window.is_open() {
            total_cycles = 0;

            while total_cycles < CYCLES_PER_FRAME / 2 {
                total_cycles += self.step() as u64;
            }

            self.request_interrupt(1);

            while total_cycles < CYCLES_PER_FRAME {
                total_cycles += self.step() as u64;
            }

            self.request_interrupt(2);
            self.handle_input();

            self.bus.video.render(&self.bus.memory);
            self.window.update_with_buffer(
                self.bus.video.framebuffer(),
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            ).unwrap();
        }
    }

    pub fn step(&mut self) -> u8 {
        let ctx = &mut CpuContext {
            bus: &mut self.bus
        };

        self.cpu.step(ctx)
    }
    
    pub fn run(&mut self) {
        loop {
            if self.cpu.halted {
                break;
            }
            
        self.run_one_frame();
        }
    }
}