use crate::simulation::ship::modules::components::thruster::Thruster;
use crate::simulation::ship::modules::{ShipModule, ShipModuleBundle, ShipModuleType};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Bundle, Component, World};
use serde::{Deserialize, Serialize};

#[derive(Debug, Bundle, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrusterBundle {
    #[serde(default, skip_serializing)]
    tag: ThrusterTag,
    pub base: ShipModule,
    pub thruster: Thruster,
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
        }
    }
}

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrusterTag;

impl ShipModuleBundle for ThrusterBundle {
    fn from_entity(entity: Entity, world: &World) -> Option<Self> {
        let base = world.get::<ShipModule>(entity)?.clone();
        let thruster = world.get::<Thruster>(entity)?.clone();
        Some(Self {
            tag: ThrusterTag,
            base,
            thruster,
        })
    }
}
