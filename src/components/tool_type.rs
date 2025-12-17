use bevy::ecs::component::Component;

use crate::components::TileType;

#[derive(Component)]
pub enum ToolType {
    PlaceTile(TileType),
}
