use crate::simulation::tags::resource_specific::add_resource_consume_tag;
use crate::simulation::types::resource_type::ResourceType;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceConsume {
    resources: HashMap<ResourceType, f64>,
    #[serde(default, skip_serializing)]
    satisfied: bool,
}

impl ResourceConsume {
    pub fn spawn_tags(&self, entity: Entity, commands: &mut Commands) {
        for resource in self.resources.keys() {
            add_resource_consume_tag(commands, entity, *resource)
        }
    }
}
