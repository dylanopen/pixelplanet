use bevy::math::IVec2;
use bevy::ecs::message::Message;

#[derive(Debug, Clone, Message)]
pub struct CreateResidentialBuildingMessage {
    pub pos: IVec2,
    pub capacity: u32,
}
