use std::thread;
use intel8080::Cpu;
use intel8080::cpu::CpuContext;
use crate::machine_bus::MachineBus;
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use crate::video::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::time::{Duration, Instant};

const CYCLES_PER_FRAME: u64 = 2_000_000 / 60;
const FRAME_DURATION: Duration = Duration::from_nanos(16_666_667);

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
                WindowOptions {
                    borderless: false,
                    transparency: false,
                    title: true,
                    resize: true,
                    scale: Scale::X2,
                    scale_mode: ScaleMode::Stretch,
                    topmost: false,
                    none: false,
                },
            ).unwrap()
        }
    }

    fn request_interrupt(&mut self, rst: u8) {
        let ctx = &mut CpuContext {
            bus: &mut self.bus
        };

        self.cpu.interrupt(ctx, rst);
    }

    fn handle_input(&mut self) {
        self.cpu.debug_enabled = self.window.is_key_down(minifb::Key::F1);
        self.cpu.halted = self.window.is_key_down(minifb::Key::Escape);
        self.bus.inputs.coin = self.window.is_key_down(minifb::Key::C);

        self.bus.inputs.start1 = self.window.is_key_down(minifb::Key::NumPad1);
        self.bus.inputs.p1_left = self.window.is_key_down(minifb::Key::A);
        self.bus.inputs.p1_right = self.window.is_key_down(minifb::Key::D);
        self.bus.inputs.p1_shoot = self.window.is_key_down(minifb::Key::W);

        self.bus.inputs.start2 = self.window.is_key_down(minifb::Key::NumPad2);
        self.bus.inputs.p2_left = self.window.is_key_down(minifb::Key::NumPad4);
        self.bus.inputs.p2_right = self.window.is_key_down(minifb::Key::NumPad6);
        self.bus.inputs.p2_shoot = self.window.is_key_down(minifb::Key::NumPad8);
    }

    fn run_one_frame(&mut self) {
        let mut total_cycles  = 0;

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

    fn step(&mut self) -> u8 {
        let ctx = &mut CpuContext {
            bus: &mut self.bus
        };

        self.cpu.step(ctx)
    }
    
    pub fn run(&mut self) {
        loop {
            let frame_start = Instant::now();

            if self.cpu.halted {
                break;
            }

        self.run_one_frame();

            let elapsed = frame_start.elapsed();

            if elapsed < FRAME_DURATION {
                thread::sleep(FRAME_DURATION - elapsed);
            }
        }
    }
}