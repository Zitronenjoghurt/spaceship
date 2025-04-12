use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thruster {
    thrust_newton: f64,
}

impl Thruster {
    pub fn get_thrust_newton(&self) -> f64 {
        self.thrust_newton
    }
}
