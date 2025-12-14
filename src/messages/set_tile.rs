use bevy::{ecs::message::Message, math::IVec2};

use crate::components::Tile;

#[derive(Message)]
pub struct SetTileMessage {
    pub pos: IVec2,
    pub tile: Tile,
}
