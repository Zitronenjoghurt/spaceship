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
    #[serde(default = "default_true")]
    active: bool,
    #[serde(default)]
    satisfied: bool,
}

fn default_true() -> bool {
    true
}

impl ResourceConsume {
    pub fn create(resources_consume: Vec<(ResourceType, f64)>, active: bool) -> Self {
        let mut resources = HashMap::new();
        for (resource_type, amount) in resources_consume {
            resources.insert(resource_type, amount);
        }

        Self {
            resources,
            active,
            satisfied: false,
        }
    }

    pub fn spawn_tags(&self, entity: Entity, commands: &mut Commands) {
        for resource in self.resources.keys() {
            add_resource_consume_tag(commands, entity, *resource)
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn is_satisfied(&self) -> bool {
        self.satisfied
    }

    pub fn set_satisfied(&mut self, satisfied: bool) {
        self.satisfied = satisfied;
    }

    pub fn get_resource_map(&self) -> &HashMap<ResourceType, f64> {
        &self.resources
    }
}
