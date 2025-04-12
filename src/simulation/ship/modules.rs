use crate::simulation::ship::modules::thruster::ThrusterBundle;
use bevy_ecs::prelude::{Component, Entity, World};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod components;
pub mod thruster;

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipModule {
    name: String,
    active: bool,
    module_type: ShipModuleType,
}

pub trait ShipModuleBundle: Sized {
    fn from_entity(entity: Entity, world: &World) -> Option<Self>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShipModuleType {
    #[default]
    None,
    Thruster,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShipModuleDefinition {
    Thruster(ThrusterBundle),
}

impl ShipModuleDefinition {
    pub fn from_entity(entity: Entity, world: &World) -> Option<Self> {
        let ship_module = world.get::<ShipModule>(entity)?;

        match ship_module.module_type {
            ShipModuleType::Thruster => {
                Some(Self::Thruster(ThrusterBundle::from_entity(entity, world)?))
            }
            ShipModuleType::None => None,
        }
    }

    pub fn spawn_bundle(&self, world: &mut World) -> Entity {
        match self {
            Self::Thruster(bundle) => world.spawn(bundle.clone()).id(),
        }
    }
}
