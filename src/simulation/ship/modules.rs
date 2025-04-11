use crate::simulation::ship::modules::thruster::ThrusterBundle;
use bevy_ecs::prelude::Component;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod thruster;

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipModule {
    name: String,
    active: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShipModuleDefinition {
    Thruster(ThrusterBundle),
}
