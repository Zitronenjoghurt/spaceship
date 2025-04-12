use crate::simulation::types::resource_type::ResourceType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceCollection(HashMap<ResourceType, f64>);

impl ResourceCollection {
    pub fn add(&mut self, resource_type: ResourceType, value: f64) {
        *self.0.entry(resource_type).or_insert(0.0) += value;
    }

    pub fn remove(&mut self, resource_type: ResourceType, value: f64) {
        *self.0.entry(resource_type).or_insert(0.0) -= value;
    }

    pub fn delete(&mut self, resource_type: ResourceType) {
        self.0.remove(&resource_type);
    }

    pub fn get(&self, resource_type: ResourceType) -> Option<f64> {
        self.0.get(&resource_type).copied()
    }

    pub fn get_mut(&mut self, resource_type: ResourceType) -> Option<&mut f64> {
        self.0.get_mut(&resource_type)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn has_resources(&self, resource_map: &HashMap<ResourceType, f64>) -> bool {
        resource_map
            .iter()
            .all(|(resource_type, amount)| self.get(*resource_type).map_or(false, |v| v >= *amount))
    }

    pub fn add_resources(&mut self, resource_map: &HashMap<ResourceType, f64>) {
        resource_map.iter().for_each(|(resource_type, amount)| {
            self.add(*resource_type, *amount);
        });
    }

    pub fn remove_resources(&mut self, resource_map: &HashMap<ResourceType, f64>) -> bool {
        if !self.has_resources(resource_map) {
            return false;
        }

        resource_map.iter().for_each(|(resource_type, amount)| {
            self.remove(*resource_type, *amount);
        });

        true
    }

    pub fn get_resource_map(&self) -> &HashMap<ResourceType, f64> {
        &self.0
    }

    pub fn get_resource_map_mut(&mut self) -> &mut HashMap<ResourceType, f64> {
        &mut self.0
    }
}
