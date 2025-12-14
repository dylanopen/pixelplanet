use bevy::{ecs::message::Message, math::IVec2};

#[derive(Message)]
pub struct UpdateTileMessage {
    pub pos: IVec2,
}

