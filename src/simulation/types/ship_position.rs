use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
/// The vague position of a module inside the ship
pub struct ShipPosition {
    pub face: ShipPositionFace,
    pub octant: ShipPositionOctant,
    pub layer: ShipPositionLayer,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ShipPositionFace {
    #[default]
    Front,
    Rear,
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ShipPositionOctant {
    #[default]
    Center,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ShipPositionLayer {
    #[default]
    Inner,
    Middle,
    Outer,
}
