use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ResourceType {
    #[default]
    None,
    Propellant,
}
