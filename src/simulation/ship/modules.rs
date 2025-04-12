use crate::simulation::ship::modules::resource_container::ResourceContainerBundle;
use crate::simulation::ship::modules::thruster::ThrusterBundle;
use bevy_ecs::prelude::{Component, Entity, World};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod components;
pub mod resource_container;
pub mod thruster;

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipModule {
    name: String,
    active: bool,
    module_type: ShipModuleType,
}

impl ShipModule {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_module_type(&self) -> ShipModuleType {
        self.module_type
    }
}

pub trait ShipModuleBundle: Sized {
    fn from_entity(entity: Entity, world: &World) -> Option<Self>;
    fn spawn(&self, world: &mut World) -> Entity;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShipModuleType {
    #[default]
    None,
    ResourceContainer,
    Thruster,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShipModuleDefinition {
    ResourceContainer(ResourceContainerBundle),
    Thruster(ThrusterBundle),
}

impl ShipModuleDefinition {
    pub fn from_entity(entity: Entity, world: &World) -> Option<Self> {
        let ship_module = world.get::<ShipModule>(entity)?;

        match ship_module.module_type {
            ShipModuleType::ResourceContainer => Some(Self::ResourceContainer(
                ResourceContainerBundle::from_entity(entity, world)?,
            )),
            ShipModuleType::Thruster => {
                Some(Self::Thruster(ThrusterBundle::from_entity(entity, world)?))
            }
            ShipModuleType::None => None,
        }
    }

    pub fn spawn_bundle(&self, world: &mut World) -> Entity {
        match self {
            Self::ResourceContainer(bundle) => bundle.spawn(world),
            Self::Thruster(bundle) => bundle.spawn(world),
        }
    }
}
