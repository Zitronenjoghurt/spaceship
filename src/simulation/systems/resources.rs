use crate::simulation::resources::tick_timer::TickTimer;
use crate::simulation::ship::modules::components::resource_consume::ResourceConsume;
use crate::simulation::ship::modules::components::resource_container::ResourceStore;
use crate::simulation::ship::modules::components::resource_produce::ResourceProduce;
use crate::simulation::ship::modules::ShipModule;
use crate::simulation::types::resource_collection::ResourceCollection;
use bevy_ecs::prelude::{Entity, Query, Res};
use std::collections::HashMap;

pub fn resource_production_consumption_system(
    produce_query: Query<(&ShipModule, &ResourceProduce)>,
    mut consume_query: Query<(&ShipModule, &mut ResourceConsume)>,
    mut store_query: Query<(&ShipModule, &mut ResourceStore)>,
    timer: Res<TickTimer>,
) {
    let delta = timer.get_delta();

    // Produce Resources
    let mut produced_resources: HashMap<Entity, ResourceCollection> = HashMap::new();
    for (base, produce) in &produce_query {
        if !base.is_active() || !produce.is_active() {
            continue;
        }

        let Some(ship_entity) = base.get_parent_ship() else {
            continue;
        };

        for (resource_type, amount) in produce.get_resource_map() {
            let resource_collection = produced_resources
                .entry(ship_entity)
                .or_insert_with(ResourceCollection::default);
            resource_collection.add(*resource_type, amount * delta)
        }
    }

    // Store produced resources
    let mut storage_amount_map: HashMap<Entity, ResourceCollection> = HashMap::new();
    for (base, mut store) in &mut store_query {
        if !base.is_active() || !store.is_open() {
            continue;
        }

        let Some(ship_entity) = base.get_parent_ship() else {
            continue;
        };

        let Some(resource_collection) = produced_resources.get_mut(&ship_entity) else {
            continue;
        };

        for resource_type in store.get_resources() {
            let Some(amount_to_be_stored) = resource_collection.get_mut(resource_type) else {
                continue;
            };

            let Some(remaining_capacity) = store.get_remaining_capacity(resource_type) else {
                continue;
            };

            if remaining_capacity <= 0f64 {
                continue;
            }

            let amount_stored = store.add_amount(resource_type, *amount_to_be_stored);
            *amount_to_be_stored -= amount_stored;

            if *amount_to_be_stored <= 0f64 {
                resource_collection.delete(resource_type);
            }
        }

        let storage_map = storage_amount_map
            .entry(ship_entity)
            .or_insert_with(ResourceCollection::default);
        storage_map.add_resources(store.get_resource_map());
    }

    // Keep track of consumed resources
    let mut consumed_amount_map: HashMap<Entity, ResourceCollection> = HashMap::new();
    for (base, mut consume) in &mut consume_query {
        if !base.is_active() || !consume.is_active() {
            continue;
        }

        let Some(ship_entity) = base.get_parent_ship() else {
            consume.set_satisfied(false);
            continue;
        };

        let Some(resource_collection) = storage_amount_map.get_mut(&ship_entity) else {
            consume.set_satisfied(false);
            continue;
        };

        let satisfied = resource_collection.remove_resources(consume.get_resource_map());
        if satisfied {
            let consume_map = consumed_amount_map
                .entry(ship_entity)
                .or_insert_with(ResourceCollection::default);
            consume_map.add_resources(consume.get_resource_map());
        }

        consume.set_satisfied(satisfied);
    }

    // Remove consumed resources
    for (base, mut store) in &mut store_query {
        if !base.is_active() {
            continue;
        };

        let Some(ship_entity) = base.get_parent_ship() else {
            continue;
        };

        let Some(resource_collection) = consumed_amount_map.get_mut(&ship_entity) else {
            continue;
        };

        for (resource_type, amount_to_remove) in resource_collection.get_resource_map_mut() {
            let removed_amount = store.remove_amount(*resource_type, *amount_to_remove);
            *amount_to_remove -= removed_amount;
        }
    }
}
