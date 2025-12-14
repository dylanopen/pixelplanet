pub mod road_variant;

pub use road_variant::RoadVariant;

use bevy::{ecs::{component::Component, resource::Resource}, math::IVec2};

use crate::consts::TILEMAP_SIZE;

pub struct Tile {
    pub tiletype: TileType,
}

pub enum TileType {
    None,
    Road(RoadVariant),
}

