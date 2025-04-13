use crate::simulation::ship::modules::components::resource_store::ResourceStore;
use crate::simulation::ship::modules::{ShipModule, ShipModuleBundle, ShipModuleType};
use crate::simulation::types::ship_position::ShipPosition;
use bevy_ecs::prelude::{Bundle, Component, Entity};
use bevy_ecs::world::World;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceContainerTag;

#[derive(Debug, Bundle, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceContainerBundle {
    #[serde(default, skip_serializing)]
    tag: ResourceContainerTag,
    pub base: ShipModule,
    pub resource_store: ResourceStore,
}

impl Default for ResourceContainerBundle {
    fn default() -> Self {
        Self {
            tag: ResourceContainerTag,
            base: ShipModule {
                module_type: ShipModuleType::Thruster,
                ..Default::default()
            },
            resource_store: ResourceStore::default(),
        }
    }
}

impl ResourceContainerBundle {
    pub fn create(
        name: &str,
        active: bool,
        position: ShipPosition,
        resource_store: ResourceStore,
    ) -> Self {
        let base = ShipModule::create(name, active, ShipModuleType::ResourceContainer, position);
        Self {
            tag: ResourceContainerTag,
            base,
            resource_store,
        }
    }

    pub fn from_entity(entity: Entity, world: &World) -> Option<Self> {
        let base = world.get::<ShipModule>(entity)?.clone();
        let resource_store = world.get::<ResourceStore>(entity)?.clone();
        Some(Self {
            tag: ResourceContainerTag,
            base,
            resource_store,
        })
    }
}

impl ShipModuleBundle for ResourceContainerBundle {
    fn spawn(&mut self, world: &mut World) -> Entity {
        let entity = world.spawn(self.clone()).id();
        self.resource_store
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
