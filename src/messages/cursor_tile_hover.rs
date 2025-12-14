use bevy::{ecs::message::Message, math::IVec2};

#[derive(Message)]
pub struct CursorTileHoverMessage {
    pub pos: IVec2,
}

