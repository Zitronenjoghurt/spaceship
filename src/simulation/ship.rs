use crate::simulation::ship::modules::thruster::{Thruster, ThrusterBundle};
use crate::simulation::ship::modules::{ShipModule, ShipModuleDefinition};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude::{Component, Entity, World};
use serde::{Deserialize, Serialize};

mod modules;

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ship {
    name: String,
    #[serde(skip)]
    modules: Vec<Entity>,
    module_definitions: Vec<ShipModuleDefinition>,
}

#[derive(Debug, Default, Bundle, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipBundle {
    ship: Ship,
}

pub fn load_ship_from_json(
    json_str: &str,
    world: &mut World,
) -> Result<Entity, Box<dyn std::error::Error>> {
    let ship_bundle: ShipBundle = serde_json::from_str(json_str)?;

    let mut module_entities = Vec::new();
    for module_def in &ship_bundle.ship.module_definitions {
        let module_entity = match module_def {
            ShipModuleDefinition::Thruster(bundle) => world.spawn(bundle.clone()).id(),
        };
        module_entities.push(module_entity);
    }

    let ship_entity = world.spawn(ship_bundle).id();
    if let Some(mut ship) = world.get_mut::<Ship>(ship_entity) {
        ship.modules = module_entities;
    }

    Ok(ship_entity)
}

pub fn save_ship_to_json(
    ship_entity: Entity,
    world: &mut World,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut ship = world.get::<Ship>(ship_entity).unwrap().clone();
    ship.module_definitions.clear();

    for &module_entity in &ship.modules {
        if let (Some(base), Some(thruster)) = (
            world.get::<ShipModule>(module_entity),
            world.get::<Thruster>(module_entity),
        ) {
            let bundle = ThrusterBundle {
                base: base.clone(),
                thruster: thruster.clone(),
            };
            ship.module_definitions
                .push(ShipModuleDefinition::Thruster(bundle))
        }
    }

    let ship_bundle = ShipBundle { ship };
    let json = serde_json::to_string_pretty(&ship_bundle)?;
    Ok(json)
}
