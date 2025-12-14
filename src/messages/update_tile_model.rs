use bevy::{ecs::message::Message, math::IVec2};

#[derive(Message)]
pub struct UpdateTileModelMessage {
    pub pos: IVec2,
}
