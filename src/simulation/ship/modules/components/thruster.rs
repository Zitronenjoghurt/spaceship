use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thruster {
    thrust_newton: f32,
    propellant_consumption: f32,
}
