use crate::simulation::types::ship_position::{
    ShipPosition, ShipPositionFace, ShipPositionLayer, ShipPositionOctant,
};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Commands;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionFront;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionRear;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionLeft;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionRight;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionTop;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionBottom;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionCenter;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionNorth;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionNorthEast;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionEast;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionSouthEast;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionSouth;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionSouthWest;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionWest;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionNorthWest;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionInner;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionMiddle;

#[derive(Debug, Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipPositionOuter;

pub fn add_ship_position_tags(
    commands: &mut Commands,
    entity: Entity,
    ship_position: ShipPosition,
) {
    match ship_position.face {
        ShipPositionFace::Front => commands.entity(entity).insert(ShipPositionFront),
        ShipPositionFace::Rear => commands.entity(entity).insert(ShipPositionRear),
        ShipPositionFace::Left => commands.entity(entity).insert(ShipPositionLeft),
        ShipPositionFace::Right => commands.entity(entity).insert(ShipPositionRight),
        ShipPositionFace::Top => commands.entity(entity).insert(ShipPositionTop),
        ShipPositionFace::Bottom => commands.entity(entity).insert(ShipPositionBottom),
    };

    match ship_position.octant {
        ShipPositionOctant::Center => commands.entity(entity).insert(ShipPositionCenter),
        ShipPositionOctant::N => commands.entity(entity).insert(ShipPositionNorth),
        ShipPositionOctant::NE => commands.entity(entity).insert(ShipPositionNorthEast),
        ShipPositionOctant::E => commands.entity(entity).insert(ShipPositionEast),
        ShipPositionOctant::SE => commands.entity(entity).insert(ShipPositionSouthEast),
        ShipPositionOctant::S => commands.entity(entity).insert(ShipPositionSouth),
        ShipPositionOctant::SW => commands.entity(entity).insert(ShipPositionSouthWest),
        ShipPositionOctant::W => commands.entity(entity).insert(ShipPositionWest),
        ShipPositionOctant::NW => commands.entity(entity).insert(ShipPositionNorthWest),
    };

    match ship_position.layer {
        ShipPositionLayer::Inner => commands.entity(entity).insert(ShipPositionInner),
        ShipPositionLayer::Middle => commands.entity(entity).insert(ShipPositionMiddle),
        ShipPositionLayer::Outer => commands.entity(entity).insert(ShipPositionOuter),
    };
}
