use crate::simulation::ship::modules::ShipModule;
use bevy_ecs::prelude::{Bundle, Component};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thruster {
    thrust_newton: f32,
    propellant_consumption: f32,
}

#[derive(Debug, Default, Bundle, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrusterBundle {
    pub base: ShipModule,
    pub thruster: Thruster,
}
