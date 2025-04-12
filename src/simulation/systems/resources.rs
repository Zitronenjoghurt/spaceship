use crate::simulation::resources::tick_timer::TickTimer;
use crate::simulation::ship::modules::components::resource_container::ResourceStore;
use crate::simulation::ship::modules::components::resource_produce::ResourceProduce;
use crate::simulation::ship::modules::ShipModule;
use crate::simulation::tags::resource_specific::ResourceTag;
use bevy_ecs::prelude::{Component, Query, Res};

pub fn resource_production_system<P: Component + ResourceTag, S: Component + ResourceTag>(
    produce_query: Query<(&ShipModule, &ResourceProduce, &P)>,
    mut store_query: Query<(&ShipModule, &mut ResourceStore, &S)>,
    timer: Res<TickTimer>,
) {
    let delta = timer.get_delta();

    for (produce_base, produce, tag) in &produce_query {
        if !produce_base.is_active() || !produce.is_active() {
            continue;
        }
        let tag = tag as &dyn ResourceTag;
        let resource_type = tag.get_resource_type();

        let Some(mut produced_amount) = produce.get_amount(resource_type) else {
            continue;
        };

        produced_amount *= delta;

        for (store_base, mut store, tag) in &mut store_query {
            if !store_base.is_active() || !store.is_open() {
                continue;
            }

            let amount_added = store.add_amount(resource_type, produced_amount);
            produced_amount -= amount_added;

            if produced_amount <= 0f64 {
                break;
            }
        }
    }
}
