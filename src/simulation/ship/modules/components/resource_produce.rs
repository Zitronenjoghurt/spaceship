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
    pub fn create(resources_produce: Vec<(ResourceType, f64)>, active: bool) -> Self {
        let mut resources = HashMap::new();
        for (resource_type, amount) in resources_produce {
            resources.insert(resource_type, amount);
        }

        Self { resources, active }
    }

    pub fn spawn_tags(&self, entity: Entity, commands: &mut Commands) {
        for resource in self.resources.keys() {
            add_resource_produce_tag(commands, entity, *resource)
        }
    }

    pub fn get_amount(&self, resource_type: ResourceType) -> Option<f64> {
        self.resources.get(&resource_type).copied()
    }

    pub fn get_resource_map(&self) -> &HashMap<ResourceType, f64> {
        &self.resources
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}
