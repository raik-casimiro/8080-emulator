use std::io::Cursor;
use std::sync::Arc;
use rodio::{Decoder, OutputStream, Sink, Source};

pub struct SoundEffects {
    fire: Arc<[u8]>,
    player_hit: Arc<[u8]>,
    enemy_hit: Arc<[u8]>,
    ufo: Arc<[u8]>,
    fleet_1: Arc<[u8]>,
    fleet_2: Arc<[u8]>,
    fleet_3: Arc<[u8]>,
    fleet_4: Arc<[u8]>,
}

impl SoundEffects {
    pub fn new() -> Self {
        Self {
            fire: std::fs::read("./space-invaders-machine/sounds/fire.wav").unwrap().into(),
            player_hit: std::fs::read("./space-invaders-machine/sounds/player_hit.wav").unwrap().into(),
            enemy_hit: std::fs::read("./space-invaders-machine/sounds/enemy_hit.wav").unwrap().into(),
            ufo: std::fs::read("./space-invaders-machine/sounds/ufo.wav").unwrap().into(),
            fleet_1: std::fs::read("./space-invaders-machine/sounds/fleet1.wav").unwrap().into(),
            fleet_2: std::fs::read("./space-invaders-machine/sounds/fleet2.wav").unwrap().into(),
            fleet_3: std::fs::read("./space-invaders-machine/sounds/fleet3.wav").unwrap().into(),
            fleet_4: std::fs::read("./space-invaders-machine/sounds/fleet4.wav").unwrap().into(),
        }
    }
}

pub struct Audio {
    stream: OutputStream,
    effects: SoundEffects,
    ufo_sink: Sink,
    port3: u8,
    port5: u8,
}

impl Audio {
    pub fn new() -> Self {
        let stream = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let ufo_sink = Sink::connect_new(stream.mixer());
        ufo_sink.set_volume(0.2);

        Self {
            stream,
            effects: SoundEffects::new(),
            ufo_sink,
            port3: 0,
            port5: 0,
        }
    }

    pub fn write_port3(&mut self, value: u8) {
        let rising = (!self.port3) & value;
        let falling = self.port3 & (!value);

        if rising & (1 << 0) != 0 {
            if let Ok(source) = Decoder::try_from(Cursor::new(self.effects.ufo.clone())) {
                self.ufo_sink.clear();
                self.ufo_sink.append(source.repeat_infinite());
                self.ufo_sink.play();
            }
        } else if falling & (1 << 0) != 0 {
            self.ufo_sink.stop();
        }

        if rising & (1 << 1) != 0 {
            self.play(&self.effects.fire);
        }
        if rising & (1 << 2) != 0 {
            self.play(&self.effects.player_hit);
        }
        if rising & (1 << 3) != 0 {
            self.play(&self.effects.enemy_hit);
        }

        self.port3 = value;
    }

    pub fn write_port5(&mut self, value: u8) {
        let rising = (!self.port5) & value;

        if rising & (1 << 0) != 0 {
            self.play(&self.effects.fleet_1);
        }
        if rising & (1 << 1) != 0 {
            self.play(&self.effects.fleet_2);
        }
        if rising & (1 << 2) != 0 {
            self.play(&self.effects.fleet_3);
        }
        if rising & (1 << 3) != 0 {
            self.play(&self.effects.fleet_4);
        }
        if rising & (1 << 4) != 0 {
            self.play(&self.effects.enemy_hit);
        }

        self.port5 = value;
    }
    
    fn play(&self, data: &Arc<[u8]>) {
        let sink = Sink::connect_new(self.stream.mixer());
        
        if let Ok(source) = Decoder::try_from(Cursor::new(data.clone())) {
            sink.set_volume(0.25);
            sink.append(source);
            sink.detach();
        }
    }
}