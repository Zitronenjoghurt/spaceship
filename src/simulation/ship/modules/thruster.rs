use crate::simulation::ship::modules::components::resource_consume::ResourceConsume;
use crate::simulation::ship::modules::components::resource_produce::ResourceProduce;
use crate::simulation::ship::modules::components::thruster::Thruster;
use crate::simulation::ship::modules::{ShipModule, ShipModuleBundle, ShipModuleType};
use bevy_ecs::prelude::{Bundle, Component, Entity, World};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrusterTag;

#[derive(Debug, Bundle, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrusterBundle {
    #[serde(default, skip_serializing)]
    tag: ThrusterTag,
    pub base: ShipModule,
    pub thruster: Thruster,
    pub resource_consume: ResourceConsume,
    // ToDo: Just for testing, remove later
    pub produce: ResourceProduce,
}

impl Default for ThrusterBundle {
    fn default() -> Self {
        Self {
            tag: ThrusterTag,
            base: ShipModule {
                module_type: ShipModuleType::Thruster,
                ..Default::default()
            },
            thruster: Thruster::default(),
            resource_consume: ResourceConsume::default(),
            produce: ResourceProduce::default(),
        }
    }
}

impl ShipModuleBundle for ThrusterBundle {
    fn from_entity(entity: Entity, world: &World) -> Option<Self> {
        let base = world.get::<ShipModule>(entity)?.clone();
        let thruster = world.get::<Thruster>(entity)?.clone();
        let resource_consume = world.get::<ResourceConsume>(entity)?.clone();
        let produce = world.get::<ResourceProduce>(entity)?.clone();
        Some(Self {
            tag: ThrusterTag,
            base,
            thruster,
            resource_consume,
            produce,
        })
    }

    fn spawn(&self, world: &mut World, ship_entity: Entity) -> Entity {
        let mut bundle = self.clone();
        bundle.base.set_parent_ship(ship_entity);

        let entity = world.spawn(bundle.clone()).id();
        bundle
            .resource_consume
            .spawn_tags(entity, &mut world.commands());
        bundle.produce.spawn_tags(entity, &mut world.commands());
        entity
    }
}
