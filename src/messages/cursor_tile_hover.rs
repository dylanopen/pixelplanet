use bevy::{ecs::message::Message, math::IVec2};

#[derive(Message)]
pub struct CursorTileHoverMessage {
    pos: IVec2,
}

