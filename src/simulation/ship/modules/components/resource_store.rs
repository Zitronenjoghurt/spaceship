use crate::simulation::tags::resource_specific::add_resource_store_tag;
use crate::simulation::types::resource_type::ResourceType;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Commands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceStore {
    resources: HashMap<ResourceType, f64>,
    capacity: HashMap<ResourceType, f64>,
    open: bool,
}

impl ResourceStore {
    pub fn create(resources_capacity: Vec<(ResourceType, f64)>, open: bool) -> Self {
        let mut resources = HashMap::new();
        let mut capacity = HashMap::new();
        for (resource_type, amount) in resources_capacity {
            resources.insert(resource_type, 0f64);
            capacity.insert(resource_type, amount);
        }

        Self {
            resources,
            capacity,
            open,
        }
    }

    pub fn spawn_tags(&self, entity: Entity, commands: &mut Commands) {
        for resource in self.resources.keys() {
            add_resource_store_tag(commands, entity, *resource)
        }
    }

    pub fn get_resources(&self) -> Vec<ResourceType> {
        self.resources.keys().copied().collect()
    }

    pub fn can_store_resource(&self, resource_type: ResourceType) -> bool {
        self.resources.contains_key(&resource_type)
    }

    pub fn get_amount(&self, resource: ResourceType) -> Option<f64> {
        self.resources.get(&resource).copied()
    }

    pub fn get_capacity(&self, resource_type: ResourceType) -> Option<f64> {
        self.capacity.get(&resource_type).copied()
    }

    pub fn get_remaining_capacity(&self, resource_type: ResourceType) -> Option<f64> {
        let capacity = self.get_capacity(resource_type)?;
        let amount = self.get_amount(resource_type)?;
        Some(capacity - amount)
    }

    pub fn set_amount(&mut self, resource: ResourceType, amount: f64) -> bool {
        if !self.capacity.contains_key(&resource) {
            return false;
        }

        self.resources.insert(resource, amount);
        true
    }

    pub fn add_amount(&mut self, resource: ResourceType, amount: f64) -> f64 {
        let Some(remaining_capacity) = self.get_remaining_capacity(resource) else {
            return 0f64;
        };

        let amount_to_add = if amount > remaining_capacity {
            remaining_capacity
        } else {
            amount
        };

        self.resources
            .get_mut(&resource)
            .map(|v| *v += amount_to_add);

        amount_to_add
    }

    pub fn remove_amount(&mut self, resource: ResourceType, amount: f64) -> f64 {
        let Some(remaining_amount) = self.get_amount(resource) else {
            return 0f64;
        };

        let amount_to_remove = if amount > remaining_amount {
            remaining_amount
        } else {
            amount
        };

        self.resources
            .get_mut(&resource)
            .map(|v| *v -= amount_to_remove);

        amount_to_remove
    }

    pub fn get_resource_map(&self) -> &HashMap<ResourceType, f64> {
        &self.resources
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn set_closed(&mut self, closed: bool) {
        self.open = closed;
    }
}
