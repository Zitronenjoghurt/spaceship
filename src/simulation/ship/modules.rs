use crate::simulation::ship::modules::resource_container::ResourceContainerBundle;
use crate::simulation::ship::modules::thruster::ThrusterBundle;
use crate::simulation::tags::ship_position_specific::add_ship_position_tags;
use crate::simulation::types::ship_position::ShipPosition;
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
    position: ShipPosition,
    #[serde(default, skip_serializing)]
    parent_ship: Option<Entity>,
}

impl ShipModule {
    pub fn create(
        name: &str,
        active: bool,
        module_type: ShipModuleType,
        position: ShipPosition,
    ) -> Self {
        Self {
            name: name.to_owned(),
            active,
            module_type,
            position,
            parent_ship: None,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_active(&self) -> bool {
        self.active && self.parent_ship.is_some()
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_module_type(&self) -> ShipModuleType {
        self.module_type
    }

    pub fn get_parent_ship(&self) -> Option<Entity> {
        self.parent_ship
    }

    pub fn set_parent_ship(&mut self, parent_entity: Entity) {
        self.parent_ship = Some(parent_entity);
    }
}

pub trait ShipModuleBundle {
    fn spawn(&mut self, world: &mut World) -> Entity;
    fn set_parent_ship(&mut self, parent_entity: Entity);
    fn get_ship_position(&self) -> ShipPosition;
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

    pub fn spawn_bundle(&self, world: &mut World, ship_entity: Entity) -> Entity {
        let bundle: &mut dyn ShipModuleBundle = match self {
            Self::ResourceContainer(bundle) => &mut bundle.clone(),
            Self::Thruster(bundle) => &mut bundle.clone(),
        };

        bundle.set_parent_ship(ship_entity);

        let entity = bundle.spawn(world);

        add_ship_position_tags(&mut world.commands(), entity, bundle.get_ship_position());

        entity
    }
}
