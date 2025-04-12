use crate::simulation::types::resource_type::ResourceType;
use bevy_ecs::prelude::{Commands, Component, Entity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropellantConsume;
impl ResourceTag for PropellantConsume {
    fn get_resource_type(&self) -> ResourceType {
        ResourceType::Propellant
    }
}

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropellantProduce;
impl ResourceTag for PropellantProduce {
    fn get_resource_type(&self) -> ResourceType {
        ResourceType::Propellant
    }
}

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropellantStore;
impl ResourceTag for PropellantStore {
    fn get_resource_type(&self) -> ResourceType {
        ResourceType::Propellant
    }
}

pub trait ResourceTag {
    fn get_resource_type(&self) -> ResourceType;
}

pub fn add_resource_consume_tag(
    commands: &mut Commands,
    entity: Entity,
    resource_type: ResourceType,
) {
    match resource_type {
        ResourceType::Propellant => commands.entity(entity).insert(PropellantConsume),
        ResourceType::None => &mut commands.entity(entity),
    };
}

pub fn add_resource_produce_tag(
    commands: &mut Commands,
    entity: Entity,
    resource_type: ResourceType,
) {
    match resource_type {
        ResourceType::Propellant => commands.entity(entity).insert(PropellantProduce),
        ResourceType::None => &mut commands.entity(entity),
    };
}

pub fn add_resource_store_tag(
    commands: &mut Commands,
    entity: Entity,
    resource_type: ResourceType,
) {
    match resource_type {
        ResourceType::Propellant => commands.entity(entity).insert(PropellantStore),
        ResourceType::None => &mut commands.entity(entity),
    };
}
