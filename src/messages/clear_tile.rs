use bevy::{ecs::message::Message, math::IVec2};

#[derive(Debug, Clone, Message)]
pub struct ClearTileMessage {
    pub pos: IVec2,
}
