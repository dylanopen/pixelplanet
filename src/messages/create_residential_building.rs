use bevy::ecs::message::Message;
use bevy::math::IVec2;

#[derive(Debug, Clone, Message)]
pub struct CreateResidentialBuildingMessage {
    pub pos: IVec2,
    pub residents: u32,
    pub capacity: u32,
}
