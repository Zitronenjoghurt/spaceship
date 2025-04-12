use crate::simulation::tags::resource_specific::add_resource_produce_tag;
use crate::simulation::types::resource_type::ResourceType;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Commands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceProduce {
    resources: HashMap<ResourceType, f64>,
    active: bool,
}

impl ResourceProduce {
    pub fn spawn_tags(&self, entity: Entity, commands: &mut Commands) {
        for resource in self.resources.keys() {
            add_resource_produce_tag(commands, entity, *resource)
        }
    }

    pub fn get_amount(&self, resource_type: ResourceType) -> Option<f64> {
        self.resources.get(&resource_type).copied()
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}
