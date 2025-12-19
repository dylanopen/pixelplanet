use bevy::ecs::component::Component;

use crate::components::TileType;

#[derive(Component, Debug, Clone)]
pub enum ToolType {
    Select,
    PlaceTile(TileType),
    DeleteTile,
}
