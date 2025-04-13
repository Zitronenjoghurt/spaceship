use crate::simulation::ship::modules::components::resource_consume::ResourceConsume;
use crate::simulation::ship::modules::components::thruster::Thruster;
use crate::simulation::ship::modules::{ShipModule, ShipModuleBundle, ShipModuleType};
use crate::simulation::types::ship_position::ShipPosition;
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
        }
    }
}

impl ThrusterBundle {
    pub fn create(
        name: &str,
        active: bool,
        position: ShipPosition,
        thrust_newton: f64,
        resource_consume: ResourceConsume,
    ) -> Self {
        let base = ShipModule::create(name, active, ShipModuleType::Thruster, position);
        let thruster = Thruster::create(thrust_newton);
        Self {
            tag: ThrusterTag,
            base,
            thruster,
            resource_consume,
        }
    }

    pub fn from_entity(entity: Entity, world: &World) -> Option<Self> {
        let base = world.get::<ShipModule>(entity)?.clone();
        let thruster = world.get::<Thruster>(entity)?.clone();
        let resource_consume = world.get::<ResourceConsume>(entity)?.clone();
        Some(Self {
            tag: ThrusterTag,
            base,
            thruster,
            resource_consume,
        })
    }
}

impl ShipModuleBundle for ThrusterBundle {
    fn spawn(&mut self, world: &mut World) -> Entity {
        let entity = world.spawn(self.clone()).id();
        self.resource_consume
            .spawn_tags(entity, &mut world.commands());
        entity
    }

    fn set_parent_ship(&mut self, parent_entity: Entity) {
        self.base.set_parent_ship(parent_entity);
    }

    fn get_ship_position(&self) -> ShipPosition {
        self.base.position
    }
}
