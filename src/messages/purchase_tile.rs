use bevy::ecs::message::Message;
use bevy::math::IVec2;

use crate::components::TileType;

#[derive(Message)]
pub struct PurchaseTileMessage {
    pub pos: IVec2,
    pub tiletype: TileType,
}
