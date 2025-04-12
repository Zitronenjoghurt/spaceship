use crate::simulation::ship::modules::components::resource_container::ResourceStore;
use crate::simulation::ship::modules::{ShipModule, ShipModuleBundle, ShipModuleType};
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

impl ShipModuleBundle for ResourceContainerBundle {
    fn from_entity(entity: Entity, world: &World) -> Option<Self> {
        let base = world.get::<ShipModule>(entity)?.clone();
        let resource_store = world.get::<ResourceStore>(entity)?.clone();
        Some(Self {
            tag: ResourceContainerTag,
            base,
            resource_store,
        })
    }

    fn spawn(&self, world: &mut World) -> Entity {
        let entity = world.spawn(self.clone()).id();
        self.resource_store
            .spawn_tags(entity, &mut world.commands());
        entity
    }
}
