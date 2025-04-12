use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct TickTimer {
    ticks_per_second: u8,
    tick_counter: u64,
    delta: f64,
}

impl TickTimer {
    pub fn new(ticks_per_second: u8, tick_counter: u64) -> Self {
        Self {
            ticks_per_second,
            tick_counter,
            delta: 1.0 / (ticks_per_second as f64),
        }
    }

    pub fn get_ticks_per_second(&self) -> u8 {
        self.ticks_per_second
    }

    pub fn set_ticks_per_second(&mut self, ticks_per_second: u8) {
        self.ticks_per_second = ticks_per_second;
        self.delta = 1.0 / (ticks_per_second as f64);
    }

    pub fn get_tick_counter(&self) -> u64 {
        self.tick_counter
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;
    }

    pub fn get_delta(&self) -> f64 {
        self.delta
    }
}
