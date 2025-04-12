use crate::simulation::resources::tick_timer::TickTimer;
use bevy_ecs::prelude::ResMut;

pub fn tick_system(mut timer: ResMut<TickTimer>) {
    timer.tick();
}
