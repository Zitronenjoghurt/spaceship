use crate::simulation::resources::tick_timer::TickTimer;
use crate::simulation::systems::resources::resource_production_system;
use crate::simulation::systems::timer::tick_system;
use crate::simulation::tags::resource_specific::{PropellantProduce, PropellantStore};
use bevy_ecs::prelude::{Schedule, World};
use std::time::{Duration, Instant};

pub mod resources;
pub mod ship;
pub mod systems;
pub mod tags;
pub mod types;

pub fn build_world(tps: u8) -> World {
    let mut world = World::new();
    world.insert_resource(TickTimer::new(tps, 0));
    world
}

pub fn build_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(tick_system);
    schedule.add_systems(resource_production_system::<PropellantProduce, PropellantStore>);
    schedule
}

pub fn run_simulation(world: &mut World, schedule: &mut Schedule) {
    loop {
        let start_time = Instant::now();
        schedule.run(world);

        let ticks_per_second = world.resource::<TickTimer>().get_ticks_per_second();
        let target_tick_duration = Duration::from_secs_f32(1.0 / ticks_per_second as f32);

        let execution_time = start_time.elapsed();
        if execution_time < target_tick_duration {
            std::thread::sleep(target_tick_duration - execution_time);
        }

        println!("Simulation execution time: {:?}", execution_time);
    }
}
