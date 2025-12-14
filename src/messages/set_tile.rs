use bevy::{ecs::message::Message, math::IVec2};

use crate::components::{Tile, TileType};

#[derive(Message)]
pub struct SetTileMessage {
    pub pos: IVec2,
    pub tiletype: TileType,
}
